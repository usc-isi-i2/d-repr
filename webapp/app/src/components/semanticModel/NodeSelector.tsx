import * as React from "react";
import { ClassId, SemanticModel } from "src/models";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Select, Spin, Form } from "antd";
import axios from "axios";
import * as _ from "lodash";

const defaultProps = {
  style: {},
  formItemProps: {},
  forbiddenNodes: new Set(),
  filterDataNode: false
};

interface Field {
  value?: string;
  validateStatus?: "error" | "success";
  errorMsg?: string;
}

interface Props {
  sm: SemanticModel;
  style: any;
  formItemProps: any;
  fieldName: string;
  value?: string;
  forbiddenNodes: Set<string>;
  filterDataNode?: boolean;
}

interface State {
  nodeId: Field;
}

export class NodeSelector extends React.Component<Props, State> {
  public static defaultProps = defaultProps;

  constructor(props: Props) {
    super(props);
    this.state = {
      nodeId: { value: props.value }
    };
  }

  public validate = (): boolean => {
    if (this.state.nodeId.validateStatus === undefined) {
      const nodeId = {
        value: this.state.nodeId.value,
        ...this.validateNotInForbiddenList(this.state.nodeId.value)
      };

      this.setState({ nodeId });
      return nodeId.validateStatus === "success";
    }
    return this.state.nodeId.validateStatus === "success";
  };

  public getValue = (): string | undefined => {
    return this.state.nodeId.value;
  };

  public render() {
    const nodes = [];

    if (this.props.filterDataNode) {
      for (const n of this.props.sm.iterClassNodes()) {
        nodes.push(<Select.Option key={n.id}>{n.id}</Select.Option>);
      }
    } else {
      for (const n of this.props.sm.iterNodes()) {
        nodes.push(<Select.Option key={n.id}>{n.id}</Select.Option>);
      }
    }

    return (
      <Form.Item label={this.props.fieldName} {...this.props.formItemProps}>
        <Select
          value={this.state.nodeId.value}
          showSearch={true}
          style={this.props.style}
          onChange={this.onChangeValue}
        >
          {nodes}
        </Select>
      </Form.Item>
    );
  }

  private onChangeValue = (value: string) => {
    this.setState({
      nodeId: { value, ...this.validateNotInForbiddenList(value) }
    });
  };

  private validateNotInForbiddenList = (val?: string) => {
    if (val !== undefined && !this.props.forbiddenNodes.has(val)) {
      return {
        validateStatus: "success" as "success",
        errorMsg: undefined
      };
    }

    return {
      validateStatus: "error" as "error",
      errorMsg: "Cannot select nodes in forbidden list"
    };
  };
}
