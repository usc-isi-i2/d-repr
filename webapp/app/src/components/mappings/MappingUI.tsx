import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import VariableList from "../layout/VariableList";
import { VariableTbl, DB, AlignmentTbl } from "src/store/types";
import { connect } from "react-redux";
import * as _ from "lodash";
import WMappingForm, { MappingForm } from "./forms/MappingForm";
import CytoscapeGraph from "../misc/graph/CytoscapeGraph";
import { DimensionAlignment, Alignment } from "src/models";
import { BugError } from "src/misc/Exception";
import { Row, Col } from "antd";

const styles = {};

interface Props extends WithStyles<typeof styles> {
  variables: VariableTbl;
  mappings: AlignmentTbl;
}

const cytoscapeSMGraphStyles = [
  {
    selector: "node",
    style: {
      label: "data(label)",
      "text-halign": "center",
      "text-valign": "center",
      width: "label",
      height: "label"
    }
  },
  {
    selector: 'node[type="variable"]',
    style: {
      backgroundColor: "#FED530",
      shape: "rectangle",
      padding: "10px"
    } as any
  },
  {
    selector: "edge",
    style: {
      width: 2,
      label: "data(label)",
      "text-background-color": "#FFFFFF",
      "text-background-opacity": 1,
      "curve-style": "unbundled-bezier",
      "line-color": "#A32B2E",
      "target-arrow-color": "#A32B2E",
      "target-arrow-shape": "none"
    }
  }
];

const cytoscapeSMGraphLayout = {
  name: "spread",
  animate: true, // Whether to show the layout as it's running
  ready: undefined, // Callback on layoutready
  stop: undefined, // Callback on layoutstop
  fit: true, // Reset viewport to fit default simulationBounds
  minDist: 10, // Minimum distance between nodes
  padding: 10, // Padding
  expandingFactor: -1.0, // If the network does not satisfy the minDist
  // criterium then it expands the network of this amount
  // If it is set to -1.0 the amount of expansion is automatically
  // calculated based on the minDist, the aspect ratio and the
  // number of nodes
  prelayout: { name: "cose" }, // Layout options for the first phase
  maxExpandIterations: 4, // Maximum number of expanding iterations
  boundingBox: undefined, // Constrain layout bounds; { x1, y1, x2, y2 } or { x1, y1, w, h }
  randomize: false // Uses random initial node positions on true
};

function getEdgeId(m: Alignment): string {
  if (m.source.indexOf(":::") !== -1) {
    throw new BugError("We assume that variable id doesn't have `:::` in it");
  }
  return `${m.source}:::${m.target}`;
}

function parseEdgeId(mappings: AlignmentTbl, edgeId: string): Alignment {
  const [var1id, var2id] = edgeId.split(":::", 2);
  let mapping = null;

  for (const m of mappings) {
    if (m.source === var1id && m.target === var2id) {
      if (mapping !== null) {
        throw new BugError("More than one mapping between two variables");
      }

      mapping = m;
    }
  }

  if (mapping === null) {
    throw new BugError("There is no mapping associated with an edge");
  }

  return mapping;
}

class MappingsUI extends React.Component<Props, object> {
  private mappingForm: React.RefObject<MappingForm>;

  constructor(props: Props) {
    super(props);
    this.mappingForm = React.createRef();
  }

  public render() {
    const nodes = _.map(this.props.variables, v => ({
      id: v.id,
      label: v.id,
      type: "variable"
    }));

    const edges = [];
    for (const m of this.props.mappings) {
      edges.push({
        id: getEdgeId(m),
        source: m.source,
        target: m.target,
        label: m instanceof DimensionAlignment ? "dimension" : "value"
      });
    }

    return (
      <React.Fragment>
        <VariableList
          variables={_.values(this.props.variables)}
          onVariableClick={this.onVariableListClick}
        />
        <br />
        <Row gutter={8}>
          <Col span={12}>
            <WMappingForm
              ref={this.mappingForm}
              variables={this.props.variables}
            />
          </Col>
          <Col span={12} style={{ marginTop: -19 }}>
            <CytoscapeGraph
              nodes={nodes}
              edges={edges}
              title="Mapping"
              styles={cytoscapeSMGraphStyles}
              layout={cytoscapeSMGraphLayout}
              onNodeClick={this.onGraphNodeClick}
              onEdgeClick={this.onGraphEdgeClick}
            />
          </Col>
        </Row>
      </React.Fragment>
    );
  }

  private onGraphNodeClick = (nodeId: string) => {
    this.onVariableListClick(nodeId);
  };

  private onGraphEdgeClick = (edgeId: string) => {
    if (this.mappingForm.current) {
      const m = parseEdgeId(this.props.mappings, edgeId);
      this.mappingForm.current.setMapping(m);
    }
  };

  // click on variable in variable list will add it to the current form
  // it will clear the form if it has been filled with two variables
  private onVariableListClick = (variableId: string) => {
    if (this.mappingForm.current) {
      if (
        this.mappingForm.current.hasVariable1() &&
        this.mappingForm.current.hasVariable2()
      ) {
        // clean the list
        this.mappingForm.current.clearMapping();
        return;
      }

      if (!this.mappingForm.current.hasVariable1()) {
        this.mappingForm.current.setVariable1(this.props.variables[variableId]);
      } else {
        this.mappingForm.current.setVariable2(this.props.variables[variableId]);
      }
    }
  };
}

function db2Props(store: DB) {
  return {
    variables: store.variables,
    mappings: store.alignments
  };
}

export default connect(db2Props)(injectStyles(styles)(MappingsUI));
