import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Variable, SemanticModel, DATA_TYPES, DataType } from "src/models";
import { Card, Select, Input, Button, Form, Spin, Tooltip } from "antd";
import * as _ from "lodash";
import axios from "axios";
import { ClassId } from "src/models/ClassId";
import { OntPredicateSelector } from "./RStringSelector";
import WClassIdSelector, { ClassIdSelector } from "./ClassIdSelector";
import memoizeOne from "memoize-one";

const styles = {};

const defaultProps = {
  onCancel: () => {
    /* do nothing */
  },
  onDiscard: (variableId: string) => {
    /* do nothing */
  },
  onSave: (formResult: SemanticTypeForm) => {
    /* do nothing */
  }
};

interface Props extends WithStyles<typeof styles> {
  variable: Variable;
  semanticModel: SemanticModel;
  onDiscard: (variableId: string) => void;
  onCancel: () => void;
  onSave: (formResult: SemanticTypeForm) => void;
}

interface State {
  dataType: DataType;
}

export interface SemanticTypeForm {
  ontClass: ClassId;
  ontPredicate: string;
  dataType: DataType;
  variableId: string;
}

class SetSemanticTypeForm extends React.Component<Props, State> {
  public static defaultProps = defaultProps;
  public state: State = {
    dataType: "xsd:string"
  };
  private classIdSelector: React.RefObject<ClassIdSelector>;
  private ontPredicateSelector: React.RefObject<OntPredicateSelector>;

  private getSType = memoizeOne((sm: SemanticModel, varId: string) =>
    this.getSType_(sm, varId)
  );

  constructor(props: Props) {
    super(props);
    this.classIdSelector = React.createRef();
    this.ontPredicateSelector = React.createRef();
  }

  public render() {
    const stype = this.getSType(
      this.props.semanticModel,
      this.props.variable.id
    );
    const dataTypes = DATA_TYPES.map(o => (
      <Select.Option key={o}>{o}</Select.Option>
    ));

    return (
      <Card className="margin-top-8">
        <Form layout="inline">
          <WClassIdSelector
            innerRef={this.classIdSelector}
            key={stype.ontClass ? stype.ontClass.id : undefined}
            sm={this.props.semanticModel}
            classId={stype.ontClass}
            style={{ minWidth: 200 }}
          />
          <OntPredicateSelector
            ref={this.ontPredicateSelector}
            style={{ minWidth: 200 }}
            value={stype.ontPredicate}
            fieldName="Predicate"
          />
          <Form.Item label="Type">
            <Select
              value={this.state.dataType}
              showSearch={true}
              style={{ minWidth: 120 }}
              onChange={this.onDataTypeChange}
            >
              {dataTypes}
            </Select>
          </Form.Item>
          <Form.Item>
            {this.props.semanticModel.hasNode(this.props.variable.id) ? (
              <Button
                className="margin-right-8"
                type="danger"
                onClick={this.onDiscard}
              >
                Dismiss
              </Button>
            ) : null}
            <Button className="margin-right-8" onClick={this.props.onCancel}>
              Cancel
            </Button>
            <Button type="primary" onClick={this.onSave}>
              Save
            </Button>
          </Form.Item>
        </Form>
      </Card>
    );
  }

  private getSType_ = (sm: SemanticModel, varId: string) => {
    const stype = sm.getDataNodeSemanticType(varId);
    if (stype === undefined) {
      return { ontClass: undefined, ontPredicate: undefined };
    }

    return stype;
  };

  private onDiscard = () => {
    this.props.onDiscard(this.props.variable.id);
  };

  private onSave = () => {
    if (
      !this.classIdSelector.current!.validate() ||
      !this.ontPredicateSelector.current!.validate()
    ) {
      return;
    }

    this.props.onSave({
      ontClass: this.classIdSelector.current!.getValue()!,
      ontPredicate: this.ontPredicateSelector.current!.getValue()!,
      dataType: this.state.dataType,
      variableId: this.props.variable.id
    });
  };

  private onDataTypeChange = (value: DataType) => {
    this.setState({
      dataType: value
    });
  };
}

export default injectStyles(styles)(SetSemanticTypeForm);
