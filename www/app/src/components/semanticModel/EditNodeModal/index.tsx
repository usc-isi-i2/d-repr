import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Variable, SemanticModel, ClassId } from "src/models";
import {
  Modal,
  Button,
  Form,
  Select,
  Spin,
  Row,
  Col,
  Menu,
  Divider
} from "antd";
import { OntClassSelector, OntPredicateSelector } from "../RStringSelector";
import { SelectParam } from "antd/lib/menu";

import { Dispatch } from "redux";
import { connect } from "react-redux";
import { NodeSelector } from "../NodeSelector";
import { BugError } from "src/misc/Exception";
import { InlineEdgeForm } from "./InlineEdgeForm";
import { smRemoveNode } from "src/store/actions";

const styles = {};
const formItemProps = {
  labelCol: { span: 6 },
  wrapperCol: { span: 18 }
};

interface Props extends WithStyles<typeof styles> {
  sm: SemanticModel;
  nodeId?: string;
  visible: boolean;
  onHide: () => void;
  dispatch: Dispatch;
}

interface State {
  activeMenu: MenuItem;
}

type MenuItem = "OntClass" | "OutgoingEdge" | "IncomingEdge";

class EditNodeModal extends React.Component<Props, State> {
  private ontClassSelector: React.RefObject<OntClassSelector>;

  constructor(props: Props) {
    super(props);
    this.ontClassSelector = React.createRef();
    this.state = {
      activeMenu: "OntClass"
    };
  }

  public render() {
    if (this.props.nodeId === undefined) {
      return null;
    }

    const node = this.props.sm.getNodeById(this.props.nodeId);
    if (!node.isClassNode()) {
      throw new BugError(
        "EditNodeModal component can only use to edit class node"
      );
    }

    let component = null;

    if (this.state.activeMenu === "OntClass") {
      component = (
        <OntClassSelector
          ref={this.ontClassSelector}
          key={this.props.nodeId}
          style={{ width: "100%" }}
          fieldName=""
          value={this.props.sm.getNodeById(this.props.nodeId).classId!.shortURI}
          formItemProps={formItemProps}
        />
      );
    } else {
      const isIncomingMenu = this.state.activeMenu === "IncomingEdge";
      const edges = isIncomingMenu
        ? this.props.sm.iterIncomingEdges(node.id)
        : this.props.sm.iterOutgoingEdges(node.id);
      component = [];
      component.push(
        <InlineEdgeForm
          key="new"
          dispatch={this.props.dispatch}
          isIncomingEdge={isIncomingMenu}
          sm={this.props.sm}
          currentNodeId={node.id}
        />
      );
      for (const e of edges) {
        component.push(
          <InlineEdgeForm
            key={e.id}
            edgeId={e.id}
            dispatch={this.props.dispatch}
            isIncomingEdge={isIncomingMenu}
            sm={this.props.sm}
            currentNodeId={node.id}
            otherNodeId={isIncomingMenu ? e.sourceId : e.targetId}
            predicate={e.predicate}
          />
        );
      }
    }

    return (
      <Modal
        title="Edit Node"
        visible={this.props.visible}
        onCancel={this.onCancel}
        width="80%"
        footer={[
          <Button type="danger" key="discard" onClick={this.onDiscard}>
            Remove
          </Button>,
          <Button key="back" onClick={this.onCancel}>
            Close
          </Button>
        ]}
      >
        <Row gutter={8}>
          <Col span={6}>
            <Menu
              mode="inline"
              onSelect={this.onChangeMenu}
              defaultSelectedKeys={["OntClass"]}
            >
              <Menu.Item key="OntClass">Ontology Class</Menu.Item>
              <Menu.Item key="IncomingEdge">Incoming Edges</Menu.Item>
              <Menu.Item key="OutgoingEdge">Outgoing Edges</Menu.Item>
            </Menu>
          </Col>
          <Col span={18}>{component}</Col>
        </Row>
      </Modal>
    );
  }

  private onChangeMenu = (selection: SelectParam) => {
    this.setState({ activeMenu: selection.key as MenuItem });
  };

  private onCancel = (e: any) => {
    this.props.onHide();
  };

  private onDiscard = (e: any) => {
    this.props.onHide();
    this.props.dispatch(smRemoveNode(this.props.nodeId!));
  };
}

export default connect()(injectStyles(styles)(EditNodeModal));
