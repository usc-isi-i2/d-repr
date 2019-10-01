import * as React from "react";
import { Row, Col, Button } from "antd";
import { SemanticModel, ClassId } from "src/models";
import { OntPredicateSelector } from "../RStringSelector";
import { NodeSelector } from "../NodeSelector";
import { Dispatch } from "redux";
import { smAddEdge, smUpdateEdge, smRemoveEdge } from "src/store/actions";

interface Props {
  sm: SemanticModel;
  currentNodeId: string;
  isIncomingEdge: boolean;
  otherNodeId?: string;
  predicate?: string;

  edgeId?: string;
  dispatch: Dispatch;
}

export class InlineEdgeForm extends React.Component<Props, object> {
  private nodeSelector: React.RefObject<NodeSelector>;
  private predicateSelector: React.RefObject<OntPredicateSelector>;

  constructor(props: Props) {
    super(props);
    this.nodeSelector = React.createRef();
    this.predicateSelector = React.createRef();
  }

  public render() {
    const node = this.props.sm.getNodeById(this.props.currentNodeId);
    let controller;
    if (this.props.otherNodeId === undefined) {
      controller = (
        <Button className="margin-top-4" onClick={this.onCreateEdge}>
          Add
        </Button>
      );
    } else {
      controller = (
        <React.Fragment>
          <Button
            className="margin-top-4 margin-right-8"
            type="danger"
            onClick={this.onRemoveEdge}
          >
            Remove
          </Button>
          <Button className="margin-top-4" onClick={this.onUpdateEdge}>
            Update
          </Button>
        </React.Fragment>
      );
    }

    return (
      <Row gutter={8}>
        <Col span={9}>
          <NodeSelector
            ref={this.nodeSelector}
            sm={this.props.sm}
            fieldName=""
            style={{ width: "100%" }}
            forbiddenNodes={new Set([node.id])}
            filterDataNode={true}
            value={this.props.otherNodeId}
          />
        </Col>
        <Col span={9}>
          <OntPredicateSelector
            key="new"
            fieldName=""
            ref={this.predicateSelector}
            style={{ width: "100%" }}
            value={this.props.predicate}
          />
        </Col>
        <Col span={6}>{controller}</Col>
      </Row>
    );
  }

  private onCreateEdge = () => {
    if (!this.hasData()) {
      return;
    }

    const [sourceId, targetId] = this.getSourceAndTarget();
    this.props.dispatch(smAddEdge(sourceId, this.getPredicate()!, targetId));
  };

  private onUpdateEdge = () => {
    if (!this.hasData()) {
      return;
    }

    const [sourceId, targetId] = this.getSourceAndTarget();
    this.props.dispatch(
      smUpdateEdge(this.props.edgeId!, sourceId, this.getPredicate()!, targetId)
    );
  };

  private onRemoveEdge = () => {
    this.props.dispatch(smRemoveEdge(this.props.edgeId!));
  };

  private hasData = (): boolean => {
    return (
      this.getPredicate() !== undefined && this.getOtherNodeId() !== undefined
    );
  };

  private getPredicate(): string | undefined {
    if (this.predicateSelector.current) {
      return this.predicateSelector.current.getValue();
    }
    return undefined;
  }

  private getOtherNodeId(): string | undefined {
    if (this.nodeSelector.current) {
      return this.nodeSelector.current.getValue();
    }
    return undefined;
  }

  private getSourceAndTarget(): [ClassId, ClassId | string] {
    const otherNode = this.props.sm.getNodeById(this.getOtherNodeId()!);
    const currentNode = this.props.sm.getNodeById(this.props.currentNodeId);

    let sourceId;
    let targetId;
    if (this.props.isIncomingEdge) {
      sourceId = otherNode.classId!;
      targetId = currentNode.classId!;
    } else {
      sourceId = currentNode.classId!;
      // otherNode is always class node because of the logic, but still do unnecessary checking
      targetId = otherNode.isClassNode() ? otherNode.classId! : otherNode.id;
    }
    return [sourceId, targetId];
  }
}
