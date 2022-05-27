import * as React from "react";
import {
  Variable,
  Location,
  TypeVariableSorted,
  TypeVariableType
} from "src/models";
import { ResourcesTbl, VariableTbl } from "src/store/types";
import {
  Select,
  Form,
  Input,
  Button,
  Card,
  Col,
  Row,
  Icon,
  Switch
} from "antd";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import * as _ from "lodash";
import { AntDFormField } from "src/misc/AntDForm";

const styles = {
  root: {
    "& > div": {
      padding: "8px !important"
    }
  }
};

interface Props extends WithStyles<typeof styles> {
  onCancel: () => void;
  onDelete: (prevID: string) => void;
  onSave: (prevID: string, v: Variable) => void;
  isInsert: boolean;
  variables: VariableTbl;
  resources: ResourcesTbl;
  variable: Variable;
  resourceId: string;
  onChangeResourceId: (resourceId: string) => void;
  layout: AntDFormField<string>;
  onChangeLayout: (layout: string) => void;
}

interface State {
  varId: AntDFormField<string>;
  type: TypeVariableType;
  unique: boolean;
  sorted: TypeVariableSorted;
  missingValues: string[];
  isCollapsed: boolean;
}

export class UpsertVariableForm extends React.Component<Props, State> {
  public state: State = {
    varId: new AntDFormField(this.props.variable.id),
    type: this.props.variable.type,
    unique: this.props.variable.unique,
    sorted: this.props.variable.sorted,
    missingValues: this.props.variable.missingValues,
    isCollapsed: true
  };

  public render() {
    const delBtn = this.props.isInsert ? null : (
      <Button
        type="danger"
        className="margin-left-8"
        onClick={this.delete}
        data-testid="upsert-variable-form-delete-btn"
      >
        Delete
      </Button>
    );
    let expandedSection = null;

    if (!this.state.isCollapsed) {
      expandedSection = (
        <Row gutter={8}>
          <Col span={2} offset={1}>
            <Form.Item
              label="Unique"
              data-testid="upsert-variable-form-variable-unique"
            >
              <Switch
                checked={this.state.unique}
                onChange={this.onUniqueChange}
              />
            </Form.Item>
          </Col>
          <Col span={3}>
            <Form.Item label="Sorted">
              <Select
                value={this.state.sorted}
                onChange={this.onSortedChange}
                data-testid="upsert-variable-form-variable-sorted"
              >
                <Select.Option
                  data-testid="antd-select-options"
                  data-testvalue="null"
                  value="null"
                >
                  No Order
                </Select.Option>
                <Select.Option
                  data-testid="antd-select-options"
                  data-testvalue="ascending"
                  value={"ascending"}
                >
                  Ascending
                </Select.Option>
                <Select.Option
                  data-testid="antd-select-options"
                  data-testvalue="descending"
                  value={"descending"}
                >
                  Descending
                </Select.Option>
              </Select>
            </Form.Item>
          </Col>
          <Col span={2}>
            <Form.Item label="Value Type">
              <Select
                value={this.state.type}
                onChange={this.onVariableTypeChange}
                data-testid="upsert-variable-form-variable-type"
              >
                {["unspecified", "int", "float", "string"].map(s => (
                  <Select.Option
                    data-testid="ant-select-options"
                    data-testvalue={s}
                    value={s}
                  >
                    {_.capitalize(s)}
                  </Select.Option>
                ))}
              </Select>
            </Form.Item>
          </Col>
          <Col span={14}>
            <Form.Item label="Missing Values">
              <Select
                mode="tags"
                data-testid="upsert-variable-form-variable-missing-values"
              >
                {_.map(this.state.missingValues, (v: string) => (
                  <Select.Option
                    key={v}
                    value={v}
                    data-testid="antd-select-options"
                    data-testvalue={v}
                  >
                    {v}
                  </Select.Option>
                ))}
              </Select>
            </Form.Item>
          </Col>
        </Row>
      );
    }

    return (
      <Card className={this.props.classes.root}>
        <Form>
          <Row gutter={8}>
            <Col span={5}>
              <Form.Item label="Resource Id">
                <Select
                  value={this.props.resourceId}
                  onChange={this.onChangeResourceId}
                  data-testid="upsert-variable-form-resource-id"
                >
                  {Object.keys(this.props.resources).map(rid => (
                    <Select.Option
                      key={rid}
                      value={rid}
                      data-testid="antd-select-options"
                      data-testvalue={rid}
                    >
                      {rid}
                    </Select.Option>
                  ))}
                </Select>
              </Form.Item>
            </Col>
            <Col span={5}>
              <Form.Item
                label="Attribute Id"
                validateStatus={this.state.varId.validationStatus}
                help={this.state.varId.validationMessage}
              >
                <Input
                  value={this.state.varId.value}
                  onChange={this.onVariableIdChange}
                  data-testid="upsert-variable-form-variable-id"
                />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Attribute Layout"
                validateStatus={this.props.layout.validationStatus}
                help={this.props.layout.validationMessage}
              >
                <Input
                  value={this.props.layout.value}
                  onChange={this.onLayoutInputChange}
                  data-testid="upsert-variable-form-variable-layout"
                />
              </Form.Item>
            </Col>
            <Col span={6} style={{ textAlign: "right" }}>
              <Form.Item label="&nbsp;" colon={false}>
                <Button
                  onClick={this.props.onCancel}
                  data-testid="upsert-variable-form-cancel-btn"
                >
                  Cancel
                </Button>
                {delBtn}
                <Button
                  className="margin-left-8"
                  onClick={this.save}
                  type="primary"
                  data-testid="upsert-variable-form-submit-btn"
                >
                  Save
                </Button>
                <a
                  style={{ marginLeft: 8, fontSize: 12 }}
                  onClick={this.toggleExpand}
                  data-testid="upsert-variable-form-collapse"
                >
                  Collapse{" "}
                  <Icon type={this.state.isCollapsed ? "up" : "down"} />
                </a>
              </Form.Item>
            </Col>
          </Row>
          {expandedSection}
        </Form>
      </Card>
    );
  }

  private delete = () => {
    this.props.onDelete(this.props.variable.id);
  };

  private save = () => {
    if (!this.props.layout.isValid() || !this.state.varId.isValid()) {
      return;
    }

    const resource = this.props.resources[this.props.resourceId];
    const layout = Location.fromString(
      resource.resource.resourceId,
      this.props.layout.value
    );

    const nv = new Variable(
      this.state.varId.value,
      this.state.sorted,
      this.state.type,
      this.state.unique,
      this.state.missingValues,
      layout
    );
    this.props.onSave(this.props.variable.id, nv);
  };

  private toggleExpand = () => {
    this.setState({ isCollapsed: !this.state.isCollapsed });
  };

  private onSortedChange = (sorted: TypeVariableSorted) => {
    this.setState({
      sorted
    });
  };

  private onChangeResourceId = (resourceId: string) => {
    this.props.onChangeResourceId(resourceId);
  };

  private onVariableTypeChange = (type: TypeVariableType) => {
    this.setState({
      type
    });
  };

  private onUniqueChange = (unique: boolean) => {
    this.setState({ unique });
  };

  private onVariableIdChange = (event: { target: HTMLInputElement }) => {
    let varId;
    const value = event.target.value;
    if (value.trim() === "") {
      varId = new AntDFormField(
        value,
        "error",
        "Attribute id cannot be left blank"
      );
    } else if (value.trim() in this.props.variables) {
      varId = new AntDFormField(value, "error", "Duplicated attribute id");
    } else {
      varId = new AntDFormField(value);
    }

    this.setState({ varId });
  };

  private onLayoutInputChange = (event: any) => {
    this.props.onChangeLayout(event.target.value);
  };
}

export default injectStyles(styles)(UpsertVariableForm);
