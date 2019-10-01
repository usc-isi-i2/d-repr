import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Row, Col, Button, Divider, Icon } from "antd";
import {
  Variable,
  RangeSlice,
  IndexSlice,
  DimensionAlignment,
  Alignment
} from "src/models";
import { Index } from "src/models/data";
import * as _ from "lodash";
import { ResourcesTbl, DB } from "src/store/types";
import { connect } from "react-redux";

const styles = {
  controller: {
    marginTop: -20,
    textAlign: "center" as "center",
    "& button": {
      paddingTop: 2,
      marginRight: 8
    },
    "& button:last-child": {
      marginRight: 0
    }
  },
  dimensionElement: {
    display: "block",
    marginBottom: 8,
    marginLeft: "auto",
    marginRight: "auto",
    zIndex: 2
  },
  root: {
    zIndex: 0
  },
  drawlingLine: {
    width: "100%",
    position: "absolute" as "absolute",
    left: 0,
    top: 0,
    zIndex: 1
  },
  variableLocation: {
    margin: 8,
    "& span": {
      padding: "5px 10px",
      fontSize: 15
    }
  }
};

const dimensionColors = ["#F08080", "#AF7AC5", "#FFC300", "#DAF7A6"];

interface Props extends WithStyles<typeof styles> {
  mapping?: DimensionAlignment;
  var1?: Variable;
  var2?: Variable;
  resources: ResourcesTbl;
  onSave: (indexMapping: DimensionAlignment) => Promise<void>;
  onDiscard: () => void;
  onClear: () => void;
}

interface AlignedDim {
  source: number;
  target: number;
}

interface State {
  alignedDims: AlignedDim[];
  // keep track of what user is selecting to map between dimensions
  selectedDimensions: [Index | undefined, Index | undefined];
  dimensionElementCentroids: [
    { [elementIndex: number]: { top: number; left: number } },
    { [elementIndex: number]: { top: number; left: number } }
  ];
}

class IndexMappingForm extends React.Component<Props, State> {
  public state: State = {
    alignedDims: this.props.mapping ? this.props.mapping.alignedDimensions : [],
    selectedDimensions: [undefined, undefined],
    dimensionElementCentroids: [[], []]
  };

  public componentDidMount = () => {
    this.setState({
      dimensionElementCentroids: [
        this.getDimensionElementCentroids(this.props.var1),
        this.getDimensionElementCentroids(this.props.var2)
      ]
    });
  };

  public componentDidUpdate = (prevProps: Props, prevState: State) => {
    if (
      prevProps.var1 !== this.props.var1 ||
      prevProps.var2 !== this.props.var2
    ) {
      // the properties were changed, and we need to re-compute the btn position and reset the state correctly
      // this way it may trigger render function twice and create phantom view in a blink, however, we cannot fix
      // this using getDeriveState because the centroids of elements cannot be computed unless they are rendered
      this.setState({
        dimensionElementCentroids: [
          this.getDimensionElementCentroids(this.props.var1),
          this.getDimensionElementCentroids(this.props.var2)
        ],
        selectedDimensions: [undefined, undefined],
        alignedDims: this.props.mapping
          ? this.props.mapping.alignedDimensions
          : []
      });
    }
  };

  public render() {
    const lines = [];
    if (
      this.state.alignedDims.length > 0 &&
      _.size(this.state.dimensionElementCentroids[0]) > 0 &&
      _.size(this.state.dimensionElementCentroids[1]) > 0
    ) {
      for (const mappedDim of this.state.alignedDims) {
        lines.push(
          <line
            key={`${mappedDim.source}--${mappedDim.target}`}
            x1={this.state.dimensionElementCentroids[0][mappedDim.source].left}
            y1={this.state.dimensionElementCentroids[0][mappedDim.source].top}
            x2={this.state.dimensionElementCentroids[1][mappedDim.target].left}
            y2={this.state.dimensionElementCentroids[1][mappedDim.target].top}
            stroke="black"
          />
        );
      }
    }

    return (
      <React.Fragment>
        <Row gutter={8}>
          <Col span={8} offset={8} className={this.props.classes.controller}>
            <Button
              shape="circle"
              disabled={!this.props.var1 || !this.props.var2}
              onClick={this.props.onClear}
            >
              <Icon type="close-circle" theme="filled" />
            </Button>
            <Button
              shape="circle"
              type="danger"
              disabled={!this.props.var1 || !this.props.var2}
              onClick={this.props.onDiscard}
            >
              <Icon type="delete" theme="filled" />
            </Button>
            <Button
              shape="circle"
              disabled={this.state.alignedDims.length > 0 ? false : true}
              onClick={this.undoLatestMappingAction}
            >
              <Icon type="undo" />
            </Button>
            <Button
              shape="circle"
              type="primary"
              disabled={!this.props.var1 || !this.props.var2}
              onClick={this.saveMapping}
            >
              <Icon type="save" theme="filled" />
            </Button>
          </Col>
        </Row>
        <Row gutter={8} className={this.props.classes.root} id="drepr-map-form">
          <Col span={9} className="text-align-center">
            {this.getVariableLocationElement(this.props.var1)}
            {this.getDimensionElements(
              0,
              this.props.var1,
              this.state.selectedDimensions[0]
            )}
          </Col>
          <Col span={9} offset={6} className="text-align-center">
            {this.getVariableLocationElement(this.props.var2)}
            {this.getDimensionElements(
              1,
              this.props.var2,
              this.state.selectedDimensions[1]
            )}
          </Col>
          <svg className={this.props.classes.drawlingLine}>{lines}</svg>
        </Row>
      </React.Fragment>
    );
  }

  private saveMapping = () => {
    // TODO: need to check if the mapping is correct
    const im = new DimensionAlignment(
      this.props.var1!.id,
      this.props.var2!.id,
      []
    );
    for (const mp of this.state.alignedDims) {
      im.addAlignedDimension(mp.source, mp.target);
    }
    this.props.onSave(im);
  };

  private undoLatestMappingAction = () => {
    this.setState({
      alignedDims: this.state.alignedDims.slice(
        0,
        this.state.alignedDims.length - 2
      )
    });
  };

  // get centroids of the dimension elements so that we can draw the line between them
  private getDimensionElementCentroids = (variable?: Variable) => {
    const centroids = {};

    if (variable !== undefined) {
      const crect = (document.getElementById(
        "drepr-map-form"
      ) as any).getBoundingClientRect();

      for (const el of document.getElementsByClassName(
        `drepr-idx-map-${variable.id}`
      ) as any) {
        const elr = el.getBoundingClientRect();
        centroids[el.dataset.dimensionelementindex] = {
          top: (elr.bottom - elr.top) / 2 + elr.top - crect.top,
          left: (elr.right - elr.left) / 2 + elr.left - crect.left
        };
      }
    }

    return centroids;
  };

  private getVariableLocationElement = (variable?: Variable) => {
    if (variable === undefined) {
      return null;
    }

    return (
      <div className={this.props.classes.variableLocation}>
        {variable.location.slices.map((s, i) => {
          return (
            <span
              key={i}
              style={{
                backgroundColor: dimensionColors[i % dimensionColors.length]
              }}
            >
              {s.toString()}
            </span>
          );
        })}
      </div>
    );
  };

  // get the HTML elements that represent the dimension
  private getDimensionElements = (
    variableIndex: number,
    variable?: Variable,
    selectedDimension?: Index
  ) => {
    const elements = [];
    if (variable !== undefined) {
      const resourceType = this.props.resources[variable.location.resourceId]
        .resource.resourceType;

      elements.push(
        variable.location.slices
          .map((s, idx) => {
            if (s instanceof IndexSlice) {
              return null;
            }

            let dimName;
            if (resourceType === "csv") {
              if (idx === 1) {
                dimName = "Column";
              } else {
                dimName = "Row";
              }
            } else {
              dimName = `Dimension ${idx}`;
            }

            return (
              <Button
                key={idx}
                type={idx === selectedDimension ? "primary" : "default"}
                className={
                  this.props.classes.dimensionElement +
                  " " +
                  `drepr-idx-map-${variable.id}`
                }
                data-dimensionelementindex={idx}
                style={{
                  backgroundColor: dimensionColors[idx % dimensionColors.length]
                }}
                onClick={this.selectVariableDimension(variableIndex, idx)}
              >
                {dimName}
              </Button>
            );
          })
          .filter(r => r !== null)
      );
    }

    return elements;
  };

  private selectVariableDimension = (
    variableIndex: number,
    dimensionIndex: number
  ) => {
    // handle when user select a dimension of a variable
    return () => {
      const otherVariableIndex = variableIndex === 1 ? 0 : 1;
      if (this.state.selectedDimensions[otherVariableIndex] !== undefined) {
        this.state.selectedDimensions[variableIndex] = dimensionIndex;
        this.state.alignedDims.push({
          source: this.state.selectedDimensions[0] as number,
          target: this.state.selectedDimensions[1] as number
        });
        this.setState({
          alignedDims: this.state.alignedDims,
          selectedDimensions: [undefined, undefined]
        });
      } else {
        this.state.selectedDimensions[variableIndex] = dimensionIndex;
        this.setState({
          selectedDimensions: this.state.selectedDimensions.slice() as [
            Index | undefined,
            Index | undefined
          ]
        });
      }
    };
  };
}

function db2Props(store: DB) {
  return {
    resources: store.resources
  };
}

export default connect(db2Props)(injectStyles(styles)(IndexMappingForm));
