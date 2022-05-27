import * as React from "react";
import { data as mdata, Slice } from "../../../models";
import Matrix, { defaultProps as MatrixDefaultProps } from "../../misc/matrix";
import { DataSlice, ContinuousRangeDataSlice } from "../../../models/data";
import { PortionOfData } from "../../../models/data/PortionOfData";
import { ResourceRecord } from "src/store/types";
import { connect } from "react-redux";
import { Dispatch } from "redux";
import { resourceFetchData } from "src/store/actions/resource";
import { Card, Icon, Row } from "antd";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import ResourcePanelHeader from "../ResourcePanelHeader";
import {
  ResourceComponentProps,
  defaultResourceComponentProps,
  ResourceComponent
} from "./interface";
import { DataView2D } from "src/components/misc/matrix/DataView2D";
import { rejects } from "assert";
import { poll } from "src/misc/Polling";

const styles = {
  root: {
    "& > div.ant-card-body": {
      paddingLeft: 0,
      paddingRight: 0
    }
  },
  panelContent: {
    // padding: "0px !important",
    margin: "-1px"
  }
};

interface Props extends WithStyles<typeof styles>, ResourceComponentProps {
  dispatch: Dispatch;
  resource: ResourceRecord;
}

interface State {
  dataview: DataView2D | null;
  enableSelection: boolean;
}

function getDefaultDataView(resourceId: string, d: mdata.NDimData) {
  if (d.pod.portionSize() === 0) {
    return null;
  }
  return DataView2D.fromData(resourceId, d);
}

/* Lifecycle:
 *   1. Initialize with a resource
 *   2. The resource doesn't have any data, then we go and fetch data from server
 *   3. The resource change its data, so that property change, and state needs to
 *      react with the change only if its dataview = null (getDerivedStateFromProps)
 */
export class CSVResourcePanel extends React.Component<Props, State>
  implements ResourceComponent {
  // this component assume that an object data and resource, once created will never change
  // it shape or type, therefore, we don't need to re-compute the shape
  public static defaultProps = defaultResourceComponentProps;
  public static getDerivedStateFromProps(props: Props, state: State) {
    if (state.dataview === null) {
      return {
        dataview: getDefaultDataView(
          props.resource.resource.resourceId,
          props.resource.data
        )
      };
    }
    return null;
  }

  public constructor(props: Props) {
    super(props);
    this.state = {
      dataview: getDefaultDataView(
        props.resource.resource.resourceId,
        props.resource.data
      ),
      enableSelection: false
    };

    if (this.state.dataview === null) {
      // fetch initial data
      this.props.dispatch(
        resourceFetchData(
          props.resource.resource.resourceId,
          DataSlice.fromDimension(
            props.resource.resource.resourceId,
            props.resource.data.dimension
          )
        )
      );
    }
  }

  public waitForInit = async () => {
    await poll(() => this.state.dataview !== null, 10000, 50);
  };

  public enableSelection = () => {
    if (this.state.enableSelection) {
      return Promise.resolve();
    }

    return new Promise<void>(resolve => {
      this.setState({ enableSelection: true }, resolve);
    });
  };

  public disableSelection = () => {
    const dataview = this.state.dataview;
    if (dataview !== null) {
      return new Promise<void>((resolve, reject) => {
        try {
          this.setState(
            {
              enableSelection: false,
              dataview: dataview.removeSelectedSlices()
            },
            resolve
          );
        } catch (error) {
          reject(error);
        }
      });
    } else {
      return Promise.resolve();
    }
  };

  public setSelectedSlices = (slices: Slice[]) => {
    if (this.state.dataview === null || !this.state.enableSelection) {
      return Promise.resolve();
    }

    const dataview = this.state.dataview;
    return new Promise<void>((resolve, reject) => {
      try {
        if (dataview.dslice.isOverlappedWithSlices(slices)) {
          // it is compatible, just need to update our selected slices
          this.setState(
            {
              dataview: new DataView2D(
                dataview.data,
                dataview.dslice,
                dataview.unboundDims,
                slices
              )
            },
            resolve
          );
        } else {
          // it is not compatible, we need to generate different data slices and fetch new data to display it
          // TODO: fix me! even though we have new data slice, it's not guarantee that this data slice and the previous unbounded dimension is compatible
          const dslice = DataSlice.fromSlices(
            this.props.resource.resource.resourceId,
            this.props.resource.data.dimension,
            slices
          );
          const unboundDims = dataview.unboundDims;

          this.props
            .dispatch(
              resourceFetchData(this.props.resource.resource.resourceId, dslice)
            )
            .then(() => {
              this.setState(
                {
                  dataview: new DataView2D(
                    this.props.resource.data,
                    dslice,
                    unboundDims,
                    slices
                  )
                },
                resolve
              );
            })
            .catch(() => {
              reject();
            });
        }
      } catch (error) {
        reject(error);
      }
    });
  };

  public render() {
    if (this.state.dataview === null) {
      // the data is not fetched
      return (
        <Card style={{ textAlign: "center" }}>
          <Icon type="loading" style={{ fontSize: 32 }} />
        </Card>
      );
    }

    return (
      <Card
        className={this.props.classes.root}
        data-testid="resource-component"
        data-testvalue={this.props.resource.resource.resourceId}
        style={{ overflowX: "scroll" }}
      >
        <ResourcePanelHeader
          resource={this.props.resource.resource}
          onDeleteResource={this.props.onDeleteResource}
        />
        <Row className={this.props.classes.panelContent}>
          <Matrix
            data={this.props.resource.data}
            dataview={this.state.dataview}
            enableSelection={this.state.enableSelection}
            onUpdateSelectedSlices={this.onUpdateSelectedSlices}
            onUpdateDataSlices={this.onUpdateDataSlices}
          />
        </Row>
      </Card>
    );
  }

  private onUpdateDataSlices = (dslice: DataSlice, dataview: DataView2D) => {
    return this.props
      .dispatch(
        resourceFetchData(this.props.resource.resource.resourceId, dslice)
      )
      .then((ndslice: DataSlice) => {
        this.setState({
          dataview: new DataView2D(
            dataview.data,
            ndslice,
            dataview.unboundDims,
            dataview.selectedSlices
          )
        });
      });
  };

  private onUpdateSelectedSlices = (dataview: DataView2D) => {
    if (this.state.enableSelection && this.state.dataview) {
      this.setState({ dataview });
      this.props.onUpdateSelectedSlices(
        this.props.resource.resource.resourceId,
        dataview.selectedSlices
      );
    }
  };
}

export default connect()(injectStyles(styles)(CSVResourcePanel));
