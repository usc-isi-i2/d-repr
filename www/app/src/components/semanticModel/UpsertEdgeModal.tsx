import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Variable, SemanticModel, ClassId } from "src/models";
import { Modal, Button, Form, Select, Spin, Row, Col, Mention } from "antd";
import WClassIdSelector, { ClassIdSelector } from "./ClassIdSelector";
import { OntClassSelector, OntPredicateSelector } from "./RStringSelector";

import { Dispatch } from "redux";
import { connect } from "react-redux";
import { NodeSelector } from "./NodeSelector";
import { smUpdateEdge, smAddEdge, smRemoveEdge } from "src/store/actions";

const styles = {};
const formItemProps = {
  labelCol: { span: 6 },
  wrapperCol: { span: 18 }
};

interface Props extends WithStyles<typeof styles> {
  sm: SemanticModel;
  edgeId?: string;
  sourceId?: string;
  targetId?: string;
  visible: boolean;
  onHide: () => void;
  dispatch: Dispatch;
}

class EditEdgeModal extends React.Component<Props, object> {
  private ontPredicateSelector: React.RefObject<OntPredicateSelector>;

  constructor(props: Props) {
    super(props);
    this.ontPredicateSelector = React.createRef();
    this.state = {};
  }

  get source() {
    return this.props.sm.getNodeById(
      this.props.edgeId !== undefined
        ? this.props.sm.getEdgeById(this.props.edgeId!).sourceId
        : this.props.sourceId!
    ).classId!;
  }

  get target() {
    return this.props.sm.getNodeById(
      this.props.edgeId !== undefined
        ? this.props.sm.getEdgeById(this.props.edgeId!).targetId
        : this.props.targetId!
    ).classId!;
  }

  public render() {
    // tslint:disable-next-line:one-variable-per-declaration
    let sourceId, predicate, targetId;
    if (this.props.edgeId === undefined) {
      if (this.props.sourceId === undefined) {
        return null;
      }

      predicate = undefined;
      sourceId = this.props.sourceId;
      targetId = this.props.targetId;
    } else {
      const edge = this.props.sm.getEdgeById(this.props.edgeId);
      predicate = edge.predicate;
      sourceId = edge.sourceId;
      targetId = edge.targetId;
    }

    const footer = [];
    if (this.props.edgeId !== undefined) {
      footer.push(
        <Button type="danger" key="discard" onClick={this.onDiscard}>
          Remove
        </Button>
      );
    }
    footer.push(
      ...[
        <Button key="back" onClick={this.onCancel}>
          Cancel
        </Button>,
        <Button type="primary" key="save" onClick={this.onSave}>
          Save
        </Button>
      ]
    );

    return (
      <Modal
        title="Edit Edge"
        visible={this.props.visible}
        onOk={this.onSave}
        onCancel={this.onCancel}
        width="60%"
        footer={footer}
      >
        <Form>
          <Form.Item label="Source" {...formItemProps}>
            {sourceId}
          </Form.Item>
          <Form.Item label="Target" {...formItemProps}>
            {targetId}
          </Form.Item>
          <OntPredicateSelector
            ref={this.ontPredicateSelector}
            style={{ width: "100%" }}
            value={predicate}
            fieldName="Predicate"
            formItemProps={formItemProps}
          />
        </Form>
      </Modal>
    );
  }

  private onSave = (e: any) => {
    if (
      !this.ontPredicateSelector.current ||
      !this.ontPredicateSelector.current.validate()
    ) {
      return;
    }

    this.props.onHide();
    if (this.props.edgeId !== undefined) {
      this.props.dispatch(
        smUpdateEdge(
          this.props.edgeId,
          this.source,
          this.getEditingPredicate()!,
          this.target
        )
      );
    } else {
      this.props.dispatch(
        smAddEdge(this.source, this.getEditingPredicate()!, this.target)
      );
    }
  };

  private onCancel = (e: any) => {
    this.props.onHide();
  };

  private onDiscard = (e: any) => {
    this.props.onHide();
    this.props.dispatch(smRemoveEdge(this.props.edgeId!));
  };

  private getEditingPredicate = () => {
    if (this.ontPredicateSelector.current) {
      return this.ontPredicateSelector.current.getValue();
    }
    return undefined;
  };
}

export default connect()(injectStyles(styles)(EditEdgeModal));
