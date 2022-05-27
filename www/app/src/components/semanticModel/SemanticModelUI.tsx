import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { connect } from "react-redux";
import { DB, VariableTbl } from "src/store/types";
import VariableList from "../layout/VariableList";
import * as _ from "lodash";
import { Card, Row, Divider, Form, Input, Button, Select } from "antd";
import memoizeOne from "memoize-one";
import * as cytoscape from "cytoscape";
import CytoscapeGraph from "../misc/graph/CytoscapeGraph";
import SetSemanticTypeForm, { SemanticTypeForm } from "./SetSemanticTypeForm";
import { SemanticModel, Node } from "src/models";
import { Dispatch } from "redux";
import { smAddDataNode, smAddEdge } from "src/store/actions";
import { smRemoveNode } from "src/store/actions/semanticModel";
import EditNodeModal from "./EditNodeModal";
import UpsertEdgeModal from "./UpsertEdgeModal";
import { string } from "prop-types";

const styles = {};

interface Props extends WithStyles<typeof styles> {
  variables: VariableTbl;
  semanticModel: SemanticModel;
  dispatch: Dispatch;
}

interface State {
  editingVariableId: string | undefined;
  showEditNodeForm: undefined | { nodeId: string };
  upsertEdgeForm: {
    isVisible: boolean;
    edgeId?: string;
    sourceId?: string;
    targetId?: string;
  };
}

const cytoscapeSMGraphStyles = [
  {
    selector: "node[label]",
    style: {
      label: "data(label)",
      "text-halign": "center",
      "text-valign": "center",
      width: "label",
      height: "label"
    }
  },
  {
    selector: 'node[type="datanode"]',
    style: {
      backgroundColor: "#FED530",
      shape: "rectangle",
      padding: "10px"
    } as any
  },
  {
    selector: 'node[type="classnode"]',
    style: {
      backgroundColor: "#D3D3D3",
      padding: "10px"
    } as any
  },
  {
    selector: "edge[label]",
    style: {
      width: 2,
      label: "data(label)",
      "text-background-color": "#FFFFFF",
      "text-background-opacity": 1,
      "curve-style": "bezier",
      "control-point-step-size": 75,
      "line-color": "#A32B2E",
      "target-arrow-color": "#A32B2E",
      "target-arrow-shape": "triangle"
    }
  }
];

const cytoscapeSMGraphLayout = {
  name: "breadthfirst",
  fit: true,
  spacingFactor: 0.7,
  directed: true
};

class SemanticModelUI extends React.Component<Props, State> {
  public state: State = {
    editingVariableId: undefined,
    upsertEdgeForm: { isVisible: false },
    showEditNodeForm: undefined
  };

  private getHasSTypesVariableIds = memoizeOne((sm: SemanticModel) =>
    this.getHasSTypesVariableIds_(sm)
  );

  constructor(props: Props) {
    super(props);
  }

  public render() {
    const nodes = [];
    const edges = [];

    for (const n of this.props.semanticModel.iterNodes()) {
      nodes.push({
        id: n.id,
        label: n.id,
        type: n.type
      });
    }

    for (const e of this.props.semanticModel.iterEdges()) {
      edges.push({
        id: e.id,
        source: e.sourceId,
        label: e.predicate,
        target: e.targetId
      });
    }

    let editingVarForm = null;
    if (this.state.editingVariableId !== undefined) {
      const v = this.props.variables[this.state.editingVariableId!];
      editingVarForm = (
        <SetSemanticTypeForm
          key={v.id}
          variable={v}
          semanticModel={this.props.semanticModel}
          onSave={this.onSaveSTypeForm}
          onCancel={this.onCancelSTypeForm}
          onDiscard={this.onDiscardSTypeForm}
        />
      );
    }

    return (
      <React.Fragment>
        <VariableList
          variables={_.values(this.props.variables)}
          onVariableClick={this.onVariableClick}
          selectedVariableId={this.state.editingVariableId}
          highlightVariableIds={this.getHasSTypesVariableIds(
            this.props.semanticModel
          )}
        />
        {editingVarForm}
        <CytoscapeGraph
          nodes={nodes}
          edges={edges}
          title="Semantic Model"
          styles={cytoscapeSMGraphStyles}
          layout={cytoscapeSMGraphLayout}
          enableEdgeDrawingCreation={true}
          onNodeRightClick={this.onGraphNodeRightClick}
          onEdgeClick={this.onGraphEdgeClick}
          canDrawEdgeBetweenNodes={this.canDrawEdgeBetweenNodes}
          onDrawNewEdgeComplete={this.onDrawNewEdgeComplete}
        />
        <EditNodeModal
          sm={this.props.semanticModel}
          nodeId={
            this.state.showEditNodeForm === undefined
              ? undefined
              : this.state.showEditNodeForm!.nodeId
          }
          visible={this.state.showEditNodeForm !== undefined}
          onHide={this.hideEditNodeForm}
        />
        <UpsertEdgeModal
          sm={this.props.semanticModel}
          edgeId={this.state.upsertEdgeForm.edgeId}
          sourceId={this.state.upsertEdgeForm.sourceId}
          targetId={this.state.upsertEdgeForm.targetId}
          visible={this.state.upsertEdgeForm.isVisible}
          onHide={this.hideEditEdgeForm}
        />
      </React.Fragment>
    );
  }

  private canDrawEdgeBetweenNodes = (sourceId: string, targetId: string) => {
    return (
      sourceId !== targetId &&
      this.props.semanticModel.hasNode(sourceId) &&
      this.props.semanticModel.hasNode(targetId) &&
      this.props.semanticModel.getNodeById(sourceId).isClassNode() &&
      this.props.semanticModel.getNodeById(targetId).isClassNode()
    );
  };

  private onDrawNewEdgeComplete = (sourceId: string, targetId: string) => {
    this.setState({
      upsertEdgeForm: {
        edgeId: undefined,
        sourceId,
        targetId,
        isVisible: true
      }
    });
    return;
  };

  private onGraphNodeRightClick = (nid: string) => {
    if (this.props.semanticModel.getNodeById(nid).isClassNode()) {
      this.setState({ showEditNodeForm: { nodeId: nid } });
    } else {
      this.setState({ editingVariableId: nid });
    }
  };

  private onGraphEdgeClick = (eid: string) => {
    this.setState({
      upsertEdgeForm: {
        ...this.state.upsertEdgeForm,
        edgeId: eid,
        isVisible: true
      }
    });
  };

  private hideEditNodeForm = () => {
    this.setState({ showEditNodeForm: undefined });
  };

  private hideEditEdgeForm = () => {
    this.setState({
      upsertEdgeForm: {
        edgeId: undefined,
        sourceId: undefined,
        targetId: undefined,
        isVisible: false
      }
    });
  };

  private onVariableClick = (variableId: string) => {
    this.setState({ editingVariableId: variableId });
  };

  private onSaveSTypeForm = (formResult: SemanticTypeForm) => {
    this.props.dispatch(
      smAddDataNode(Node.datanode(formResult.variableId, formResult.dataType))
    );
    this.props.dispatch(
      smAddEdge(
        formResult.ontClass,
        formResult.ontPredicate,
        formResult.variableId
      )
    );
    this.setState({ editingVariableId: undefined });
  };

  private onCancelSTypeForm = () => {
    this.setState({ editingVariableId: undefined });
  };

  private onDiscardSTypeForm = (varId: string) => {
    this.setState({ editingVariableId: undefined });
    this.props.dispatch(smRemoveNode(varId));
  };

  private getHasSTypesVariableIds_ = (sm: SemanticModel) => {
    const dnodeIds = new Set();
    for (const n of sm.iterDataNodes()) {
      dnodeIds.add(n.id);
    }
    return dnodeIds;
  };
}

function mapDBToProps(store: DB) {
  return {
    variables: store.variables,
    semanticModel: store.semanticModel
  };
}

export default connect(mapDBToProps)(injectStyles(styles)(SemanticModelUI));
