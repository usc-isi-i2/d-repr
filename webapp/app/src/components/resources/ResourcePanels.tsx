import * as React from "react";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import { ResourcesTbl, DB, UIConfiguration } from "src/store/types";
import { Dispatch } from "redux";
import { connect } from "react-redux";
import { Button, Card, Divider } from "antd";
import CSVResourcePanel from "./components/CSVResource";
import {
  ResourceComponentProps,
  defaultResourceComponentProps,
  ResourceComponent
} from "./components/interface";
import memoizeOne from "memoize-one";
import * as _ from "lodash";
import { resourceDelete } from "src/store/actions/resource";
import { Slice } from "src/models";

const styles = {
  hideResourcePanels: {
    "& > div": {
      padding: "8px !important"
    },
    "& button": {
      marginRight: 8
    },
    "& .resource-list-header": {
      display: "inline-block",
      minWidth: 80,
      fontWeight: 600
    },
    "& .ant-divider": {
      height: "1.5em"
    },
    marginBottom: 8
  }
};

interface Props extends WithStyles<typeof styles>, ResourceComponentProps {
  resources: ResourcesTbl;
  displayMax1Resource?: boolean;
  uiConf: UIConfiguration;
  dispatch: Dispatch;
  displayingResources?: string[];
  onDisplayingResourcesChange: (displayingResources: string[]) => void;
}

interface State {
  displayingResources: string[];
}

export class ResourcePanels extends React.Component<Props, State> {
  public static defaultProps = {
    ...defaultResourceComponentProps,
    displayMax1Resource: undefined,
    onDisplayingResourcesChange: (displayingResources: string[]) => {
      /* do nothing */
    }
  };

  public state: State = {
    displayingResources: this.props.displayingResources || []
  };

  private resourceComponents: {
    [rid: string]: React.RefObject<ResourceComponent>;
  } = {};
  private postInit = memoizeOne((rs: ResourcesTbl) => this.postInit_(rs));

  public waitForInit = async (resourceId: string) => {
    if (this.resourceComponents[resourceId].current) {
      await this.resourceComponents[resourceId].current!.waitForInit();
    }
  };

  // enable selection on a specific resource
  // the reason we are using this API is because if enableSection is a property of a component
  // when the property change (enable => disable) we need to be able to detect the change
  // so that we can clear information such as selected location (which is also another information
  // that outside component can control), and it makes the logic is even more complicated
  public enableSelection = async (resourceId: string) => {
    if (this.resourceComponents[resourceId].current) {
      return await this.resourceComponents[
        resourceId
      ].current!.enableSelection();
    } else {
      throw new Error(
        "Cannot enable selection on a unmounted resource. This could happened when the displaying resources are controlled outside, and this function is called immediately after updating state"
      );
    }
  };

  // disable selection on a specific resource
  public disableSelection = async (resourceId: string) => {
    if (this.resourceComponents[resourceId].current) {
      return await this.resourceComponents[
        resourceId
      ].current!.disableSelection();
    }
    // unmounted is fine, so we don't need to disable
    return;
  };

  public setSelectedSlices = async (resourceId: string, slices: Slice[]) => {
    if (this.resourceComponents[resourceId].current) {
      return await this.resourceComponents[
        resourceId
      ].current!.setSelectedSlices(slices);
    }
    return;
  };

  // get currently open resource
  public getOpenedResources = (): string[] => {
    if (this.props.displayingResources) {
      return this.props.displayingResources;
    }
    return this.state.displayingResources;
  };

  public render() {
    this.postInit(this.props.resources);

    const openResourcePanels = [];
    const minimizedResourcePanels = [];
    const displayingResources = new Set(
      this.props.displayingResources || this.state.displayingResources
    );

    for (const resourceId in this.props.resources) {
      const resource = this.props.resources[resourceId];
      let btnType: "primary" | "default";
      if (displayingResources.has(resourceId)) {
        openResourcePanels.push(
          <CSVResourcePanel
            innerRef={this.resourceComponents[resourceId]}
            key={resourceId}
            resource={resource}
            onHideResourcePanel={this.onHideResourcePanel(resourceId)}
            onUpdateSelectedSlices={this.props.onUpdateSelectedSlices}
            onDeleteResource={this.onDeleteResource}
          />
        );
        btnType = "primary";
      } else {
        btnType = "default";
      }

      const btn = (
        <Button
          key={resourceId}
          onClick={this.onHideResourcePanel(resourceId)}
          type={btnType}
          data-testid="resource-panels-minimized-resources"
          data-testvalue={resourceId}
        >
          {resourceId}
        </Button>
      );
      minimizedResourcePanels.push(btn);
    }

    return (
      <React.Fragment>
        <Card className={this.props.classes.hideResourcePanels}>
          <div className="resource-list-header">Resources</div>
          <Divider type="vertical" />
          {minimizedResourcePanels}
        </Card>
        {openResourcePanels}
      </React.Fragment>
    );
  }

  private postInit_ = (rs: ResourcesTbl) => {
    const delIds = [];
    for (const rid in this.resourceComponents) {
      if (!(rid in rs)) {
        delIds.push(rid);
      }
    }

    for (const rid of delIds) {
      delete this.resourceComponents[rid];
    }

    for (const rid in rs) {
      if (!(rid in this.resourceComponents)) {
        this.resourceComponents[rid] = React.createRef();
      }
    }
  };

  private onDeleteResource = (resourceId: string) => {
    this.props.dispatch(resourceDelete(resourceId));
  };

  private onHideResourcePanel = (resourceID: string) => {
    return () => {
      let displayingResources: string[];
      const displayMax1Resource =
        this.props.displayMax1Resource === undefined
          ? this.props.uiConf.displayMax1Resource
          : this.props.displayMax1Resource;

      if (displayMax1Resource) {
        // hide all opened resources
        displayingResources = [resourceID];
      } else {
        displayingResources = [
          ...(this.props.displayingResources || this.state.displayingResources),
          resourceID
        ];
      }

      if (this.props.displayingResources) {
        this.props.onDisplayingResourcesChange(displayingResources);
      } else {
        this.setState({ displayingResources });
      }
    };
  };
}

function mapStateToProps(store: DB) {
  return {
    resources: store.resources,
    uiConf: store.uiConf
  };
}

export default connect(mapStateToProps)(injectStyles(styles)(ResourcePanels));
