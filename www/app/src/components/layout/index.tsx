import * as React from "react";
import { ResourcesTbl, DB, VariableTbl } from "../../store/types";
import { connect } from "react-redux";
import { Dispatch } from "redux";
import WUpsertVariableForm, { UpsertVariableForm } from "./UpsertVariableForm";
import { Variable, Location, Slice } from "src/models";
import * as _ from "lodash";
import { variableUpsert, variableDelete } from "src/store/actions";
import { Button, Layout } from "antd";
import WResourcePanels, { ResourcePanels } from "../resources/ResourcePanels";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import VariableList from "./VariableList";
import { AntDFormField } from "src/misc/AntDForm";

const styles = {
  root: {
    "&>*": { marginBottom: 10 }
  },
  innerSpace7: {
    "&>*": {
      marginLeft: 7
    },
    "&>*:first-child": { marginLeft: 0 }
  },
  controlUnit: {
    "&>*": {
      marginLeft: 7
    },
    "&>*:first-child": { marginLeft: 0 }
  }
};

interface Props extends WithStyles<typeof styles> {
  resources: ResourcesTbl;
  dispatch: Dispatch;
  variables: VariableTbl;
}

interface State {
  upsertVariable: Variable | null;
  upsertVariableResourceId?: [string]; // array but must only contain 1 variable, just to make it work easy with resource panel
  upsertVariableLayout?: AntDFormField<string>;
}

export class LayoutUI extends React.Component<Props, State> {
  public state: State = {
    upsertVariable: null,
    upsertVariableResourceId: undefined,
    upsertVariableLayout: undefined
  };
  private createVariableForm: React.RefObject<UpsertVariableForm>;
  private resourcePanels: React.RefObject<ResourcePanels>;

  constructor(props: Props) {
    super(props);
    this.createVariableForm = React.createRef();
    this.resourcePanels = React.createRef();
    this.syncLayout = _.debounce(this.syncLayout, 150);
  }

  public render() {
    const createVarForm = this.state.upsertVariable ? (
      <WUpsertVariableForm
        key={this.state.upsertVariable.id}
        isInsert={!(this.state.upsertVariable.id in this.props.variables)}
        onCancel={this.cancelUpsertVariableForm}
        onDelete={this.onDeleteVariable}
        onSave={this.onSaveVariable}
        innerRef={this.createVariableForm}
        variable={this.state.upsertVariable}
        variables={this.props.variables}
        resources={this.props.resources}
        resourceId={this.state.upsertVariableResourceId![0]}
        layout={this.state.upsertVariableLayout!}
        onChangeResourceId={this.onUpsertVariableChangeResourceId}
        onChangeLayout={this.onUpsertVariableChangeLayout}
      />
    ) : null;

    return (
      <div className={this.props.classes.root} data-testid="layout-page">
        <div className={this.props.classes.controlUnit}>
          <Button>Add preprocessing</Button>
          <Button
            onClick={this.onCreateVariable}
            type="primary"
            data-testid="create-variable-button"
          >
            Add attribute
          </Button>
        </div>
        <VariableList
          variables={_.values(this.props.variables)}
          selectedVariableId={
            this.state.upsertVariable ? this.state.upsertVariable.id : undefined
          }
          onVariableClick={this.onUpdateVariable}
        />
        {createVarForm}
        <WResourcePanels
          innerRef={this.resourcePanels}
          displayMax1Resource={true}
          // don't control displaying resources when there is no selected variable
          displayingResources={this.state.upsertVariableResourceId}
          onDisplayingResourcesChange={this.onDisplayingResourcesChange}
          onUpdateSelectedSlices={this.onUpdateSelectedSlices}
        />
      </div>
    );
  }

  private onCreateVariable = () => {
    let newVariable = null;
    if (!this.resourcePanels.current) {
      // doesn't have any effect when there is no resource panels
      return;
    }

    // set the resource id to the currently open resource panel or the first resources
    const resourceIds = this.resourcePanels.current.getOpenedResources();
    const resourceId =
      resourceIds.length > 0
        ? resourceIds[0]
        : Object.keys(this.props.resources)[0];

    const location = new Location(resourceId, []);
    newVariable = Variable.default(
      `var_${_.size(this.props.variables)}`,
      location
    );
    this.setState(
      {
        upsertVariable: newVariable,
        upsertVariableLayout: this.validateUpsertVariableLayoutField(""),
        upsertVariableResourceId: [resourceId]
      },
      () => {
        this.resourcePanels.current!.enableSelection(resourceId);
      }
    );
  };

  private onUpdateVariable = (vid: string) => {
    if (vid in this.props.variables) {
      const v = this.props.variables[vid];
      this.setState(
        {
          upsertVariable: v,
          upsertVariableLayout: new AntDFormField(v.location.toString(true)),
          upsertVariableResourceId: [v.location.resourceId]
        },
        () => {
          this.resourcePanels
            .current!.enableSelection(v.location.resourceId)
            .then(() => {
              return this.resourcePanels.current!.waitForInit(
                v.location.resourceId
              );
            })
            .then(() => {
              this.resourcePanels.current!.setSelectedSlices(
                v.location.resourceId,
                v.location.slices
              );
            });
        }
      );
    }
  };

  private onDeleteVariable = (prevID: string) => {
    const resourceId = (this.state.upsertVariable as Variable).location
      .resourceId;
    if (this.resourcePanels.current) {
      this.resourcePanels.current.disableSelection(resourceId);
    }
    this.props.dispatch(variableDelete(prevID));
    this.setState({ upsertVariable: null });
  };

  private cancelUpsertVariableForm = () => {
    // set the new variable to null and disable selection
    const resourceId = (this.state.upsertVariable as Variable).location
      .resourceId;
    if (this.resourcePanels.current) {
      this.resourcePanels.current.disableSelection(resourceId);
    }
    this.setState({
      upsertVariable: null,
      upsertVariableLayout: undefined,
      upsertVariableResourceId: undefined
    });
  };

  private onSaveVariable = (prevID: string, nv: Variable) => {
    if (prevID in this.props.variables) {
      // update variable
      this.props.dispatch(variableUpsert(nv, prevID));
    } else {
      this.props.dispatch(variableUpsert(nv));
    }

    if (this.resourcePanels.current) {
      this.resourcePanels.current.disableSelection(nv.location.resourceId);
    }
    this.setState({ upsertVariable: null });
  };

  private onDisplayingResourcesChange = (displayingResources: string[]) => {
    if (this.state.upsertVariable !== null && this.createVariableForm.current) {
      if (displayingResources.length > 0) {
        // reset the layout because resource change
        const resourceId = displayingResources[0];
        const upsertVariableLayout = this.validateUpsertVariableLayoutField("");
        this.setState(
          {
            upsertVariable: this.state.upsertVariable.clone(),
            upsertVariableLayout,
            upsertVariableResourceId: [resourceId]
          },
          () => {
            this.resourcePanels.current!.enableSelection(resourceId);
          }
        );
      }
    }
  };

  private onUpdateSelectedSlices = (resourceId: string, slices: Slice[]) => {
    if (
      this.state.upsertVariable &&
      this.createVariableForm.current &&
      resourceId === this.state.upsertVariableResourceId![0]
    ) {
      const layout = new AntDFormField(
        new Location(resourceId, slices).toString(true)
      );
      this.setState({ upsertVariableLayout: layout });
    }
  };

  private onUpsertVariableChangeResourceId = (
    upsertVariableResourceId: string
  ) => {
    this.setState(
      { upsertVariableResourceId: [upsertVariableResourceId] },
      () => {
        this.resourcePanels.current!.enableSelection(upsertVariableResourceId);
      }
    );
  };

  private onUpsertVariableChangeLayout = (layout: string) => {
    const upsertVariableLayout = this.validateUpsertVariableLayoutField(layout);

    if (upsertVariableLayout.isValid()) {
      this.setState(
        {
          upsertVariableLayout
        },
        () => {
          this.syncLayout(
            this.state.upsertVariableResourceId![0],
            upsertVariableLayout.value
          );
        }
      );
    } else {
      this.setState({
        upsertVariableLayout
      });
    }
  };

  private syncLayout = (resourceId: string, layout: string) => {
    this.resourcePanels.current!.enableSelection(resourceId).then(() => {
      this.resourcePanels.current!.setSelectedSlices(
        resourceId,
        Location.fromString(resourceId, layout).slices
      );
    });
  };

  private validateUpsertVariableLayoutField = (
    upsertVariableLayout: string
  ): AntDFormField<string> => {
    if (upsertVariableLayout.trim() === "") {
      return new AntDFormField(
        upsertVariableLayout,
        "error",
        "Layout cannot be left blank"
      );
    }

    const resource = this.props.resources[
      this.state.upsertVariableResourceId![0]
    ];
    try {
      const loc = Location.fromString(
        resource.resource.resourceId,
        upsertVariableLayout
      );

      if (!loc.validate(resource.data.dimension)) {
        return new AntDFormField(
          upsertVariableLayout,
          "error",
          "Invalid layout"
        );
      }
    } catch (e) {
      return new AntDFormField(upsertVariableLayout, "error", "Invalid format");
    }

    return new AntDFormField(upsertVariableLayout);
  };
}

function mapStateToProps(store: DB) {
  return {
    resources: store.resources,
    variables: store.variables
  };
}
export default connect(mapStateToProps)(injectStyles(styles)(LayoutUI));
