import * as React from "react";
import { VariableTbl } from "src/store/types";
import {
  Variable,
  Alignment,
  ValueAlignment,
  DimensionAlignment,
  AlignmentImpl
} from "src/models";
import MappingFormHeader, { MappingType } from "./MappingFormHeader";
import IndexMappingForm from "./IndexMappingForm";
import { Card } from "antd";
import { connect } from "react-redux";
import { Dispatch } from "redux";
import { alignmentUpsert, alignmentRemove } from "src/store/actions";
import ValueMappingForm from "./ValueMappingForm";

interface Props {
  dispatch: Dispatch;
  variables: VariableTbl;
}

interface State {
  mapping?: Alignment;
  var1id?: string;
  var2id?: string;
  mappingType: MappingType;
}

function getDefaultState(): State {
  return {
    mapping: undefined,
    var1id: undefined,
    var2id: undefined,
    mappingType: "index"
  };
}

export class MappingForm extends React.Component<Props, State> {
  public state = getDefaultState();

  public hasVariable1 = () => {
    return this.state.var1id !== undefined;
  };

  public hasVariable2 = () => {
    return this.state.var2id !== undefined;
  };

  public setMapping = (m: Alignment) => {
    window.console.log("set mapping", m);
    this.setState({
      mapping: m,
      var1id: m.source,
      var2id: m.target,
      mappingType: m instanceof DimensionAlignment ? "index" : "value"
    });
  };

  public setVariable1 = (var1: Variable) => {
    if (var1.id !== this.state.var2id) {
      this.setState({ var1id: var1.id });
    }
  };

  public setVariable2 = (var2: Variable) => {
    if (var2.id !== this.state.var1id) {
      this.setState({ var2id: var2.id });
    }
  };

  public clearMapping = () => {
    this.setState(getDefaultState());
  };

  public render() {
    let comp = null;

    if (this.state.mappingType === "index") {
      comp = (
        <IndexMappingForm
          key={this.state.mapping ? this.state.mapping.id : undefined}
          mapping={this.state.mapping as DimensionAlignment}
          var1={
            this.state.var1id
              ? this.props.variables[this.state.var1id]
              : undefined
          }
          var2={
            this.state.var2id
              ? this.props.variables[this.state.var2id]
              : undefined
          }
          onSave={this.onSaveMappingForm}
          onDiscard={this.onDiscardMappingForm}
          onClear={this.onClearMappingForm}
        />
      );
    } else {
      comp = (
        <ValueMappingForm
          key={this.state.mapping ? this.state.mapping.id : undefined}
          mapping={this.state.mapping as ValueAlignment}
          var1={
            this.state.var1id
              ? this.props.variables[this.state.var1id]
              : undefined
          }
          var2={
            this.state.var2id
              ? this.props.variables[this.state.var2id]
              : undefined
          }
          onSave={this.onSaveMappingForm}
          onDiscard={this.onDiscardMappingForm}
          onClear={this.onClearMappingForm}
        />
      );
    }

    return (
      <Card size="small">
        <MappingFormHeader
          variables={this.props.variables}
          var1id={this.state.var1id}
          var2id={this.state.var2id}
          mappingType={this.state.mappingType}
          onMappingTypeChange={this.changeMappingType}
          onSelectVar1={this.setVariable1}
          onSelectVar2={this.setVariable2}
        />
        {comp}
      </Card>
    );
  }

  private onClearMappingForm = () => {
    this.setState({
      ...getDefaultState(),
      mappingType: this.state.mappingType
    });
  };

  private onDiscardMappingForm = () => {
    if (this.state.mapping) {
      // issue a delete request
      this.props.dispatch(alignmentRemove(this.state.mapping));
    }
    // reset the data
    this.setState(getDefaultState());
  };

  private onSaveMappingForm = (mapping: Alignment): Promise<void> => {
    this.props.dispatch(alignmentUpsert(mapping));
    this.setState(getDefaultState());
    return new Promise(() => {
      return;
    });
  };

  private changeMappingType = (mappingType: MappingType) => {
    this.setState({ mappingType });
  };
}

export default connect(
  null,
  null,
  null,
  { forwardRef: true } as any
)(MappingForm);
