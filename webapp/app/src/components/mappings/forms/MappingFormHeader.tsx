import * as React from "react";
import { VariableTbl } from "src/store/types";
import { Variable } from "src/models";
import { Row, Col, Card, Select, Button, Icon } from "antd";
import { VariableSelector } from "../VariableSelector";

export type MappingType = "index" | "value";

interface Props {
  variables: VariableTbl;
  mappingType: MappingType;
  var1id?: string;
  var2id?: string;
  onMappingTypeChange: (mappingType: MappingType) => void;
  onSelectVar1: (var1: Variable) => void;
  onSelectVar2: (var2: Variable) => void;
}

export default class MappingFormHeader extends React.Component<Props, object> {
  public render() {
    return (
      <Row gutter={8}>
        <Col span={9}>
          <VariableSelector
            key={this.props.var1id}
            variables={this.props.variables}
            fieldName=""
            value={this.props.var1id}
            style={{ width: "100%" }}
            onChangeVariable={this.props.onSelectVar1}
          />
        </Col>
        <Col span={6} style={{ textAlign: "center", paddingTop: 3 }}>
          <Select
            value={this.props.mappingType}
            onChange={this.props.onMappingTypeChange}
          >
            <Select.Option value="index">Dimension</Select.Option>
            <Select.Option value="value">Value</Select.Option>
          </Select>
        </Col>
        <Col span={9}>
          <VariableSelector
            key={this.props.var2id}
            variables={this.props.variables}
            fieldName=""
            value={this.props.var2id}
            style={{ width: "100%" }}
            onChangeVariable={this.props.onSelectVar2}
          />
        </Col>
      </Row>
    );
  }
}
