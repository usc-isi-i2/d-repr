import * as React from "react";
import { ClassId, SemanticModel, Variable } from "src/models";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Select, Spin, Form } from "antd";
import axios from "axios";
import * as _ from "lodash";
import { VariableTbl } from "src/store/types";

const defaultProps = {
  style: {},
  formItemProps: {},
  forbiddenNodes: new Set(),
  onChangeVariable: (v: Variable) => {
    /* do nothing */
  }
};

interface Field {
  value?: string;
  validateStatus?: "error" | "success";
  errorMsg?: string;
}

interface Props {
  variables: VariableTbl;
  style: any;
  formItemProps: any;
  fieldName: string;
  value?: string;
  forbiddenNodes: Set<string>;
  onChangeVariable: (v: Variable) => void;
}

interface State {
  field: Field;
}

export class VariableSelector extends React.Component<Props, State> {
  public static defaultProps = defaultProps;

  constructor(props: Props) {
    super(props);
    this.state = {
      field: { value: props.value }
    };
  }

  public validate = (): boolean => {
    if (this.state.field.validateStatus === undefined) {
      const nodeId = {
        value: this.state.field.value,
        ...this.validateNotInForbiddenList(this.state.field.value)
      };

      this.setState({ field: nodeId });
      return nodeId.validateStatus === "success";
    }
    return this.state.field.validateStatus === "success";
  };

  public getValue = () => {
    return this.state.field;
  };

  public render() {
    const options = [];

    for (const vid in this.props.variables) {
      options.push(<Select.Option key={vid}>{vid}</Select.Option>);
    }

    return (
      <Form.Item label={this.props.fieldName} {...this.props.formItemProps}>
        <Select
          value={this.state.field.value}
          showSearch={true}
          style={this.props.style}
          onChange={this.onChangeValue}
        >
          {options}
        </Select>
      </Form.Item>
    );
  }

  private onChangeValue = (value: string) => {
    this.setState({
      field: { value, ...this.validateNotInForbiddenList(value) }
    });
    this.props.onChangeVariable(this.props.variables[value]);
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
