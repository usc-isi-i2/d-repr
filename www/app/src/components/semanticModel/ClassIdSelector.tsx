import * as React from "react";
import { ClassId, SemanticModel } from "src/models";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Select, Spin, Form } from "antd";
import axios from "axios";
import * as _ from "lodash";

const styles = {};

export interface ClassIdField {
  value?: ClassId;
  validateStatus?: "error" | "success";
  errorMsg?: string;
}

interface Props extends WithStyles<typeof styles> {
  sm: SemanticModel;
  style?: any;
  classId?: ClassId;
}
interface State {
  searchOntClassResults: ClassId[];
  searchingOntClasses: boolean;
  classId: ClassIdField;
}

export class ClassIdSelector extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.onSearchOntClass = _.debounce(this.onSearchOntClass, 200);

    this.state = {
      searchOntClassResults: [],
      searchingOntClasses: false,
      classId: { value: props.classId }
    };
  }

  public validate = (): boolean => {
    if (this.state.classId.validateStatus === undefined) {
      const classId = {
        value: this.state.classId.value,
        ...this.validateClassIdField(this.state.classId.value)
      };
      this.setState({ classId });

      return classId.validateStatus === "success";
    }

    return this.state.classId.validateStatus === "success";
  };

  public getValue = (): ClassId | undefined => {
    return this.state.classId.value;
  };

  public render() {
    const searchOntClassResults = this.state.searchOntClassResults.map(
      (o, i) => {
        let isNew = i === this.state.searchOntClassResults.length - 1;
        if (
          i < this.state.searchOntClassResults.length - 1 &&
          this.state.searchOntClassResults[i + 1].shortURI !== o.shortURI
        ) {
          isNew = true;
        }

        if (isNew) {
          return (
            <Select.Option key={o.serialize2str()}>
              <i>{o.label}</i>
            </Select.Option>
          );
        }

        return <Select.Option key={o.serialize2str()}>{o.label}</Select.Option>;
      }
    );

    return (
      <Form.Item
        label="ClassId"
        validateStatus={this.state.classId.validateStatus}
        help={this.state.classId.errorMsg}
      >
        <Select
          labelInValue={true}
          showSearch={true}
          value={
            this.state.classId.value
              ? { key: this.state.classId.value.serialize2str() }
              : undefined
          }
          defaultActiveFirstOption={false}
          notFoundContent={
            this.state.searchingOntClasses ? (
              <span style={{ marginLeft: 10, marginRight: 10 }}>
                <Spin size="small" />
              </span>
            ) : (
              <span style={{ marginLeft: 10, marginRight: 10 }}>
                Not Found..
              </span>
            )
          }
          showArrow={false}
          style={this.props.style}
          onSearch={this.onSearchOntClass}
          onChange={this.onChangeOntClassValue}
        >
          {searchOntClassResults}
        </Select>
      </Form.Item>
    );
  }

  private onSearchOntClass = (value: string) => {
    this.setState({ searchingOntClasses: true });
    axios
      .get("/ontologies/search", { params: { a: "class", q: value } })
      .then((resp: any) => {
        const searchOntClassResults = [];
        for (const shortURI of resp.data.results) {
          const existingNodes = this.props.sm.getClassNodesByShortURI(shortURI);

          if (existingNodes.length > 0) {
            for (const n of existingNodes) {
              searchOntClassResults.push(n.classId!);
            }

            searchOntClassResults.push(
              existingNodes[existingNodes.length - 1].classId!.next()
            );
          } else {
            searchOntClassResults.push(new ClassId(shortURI, 1));
          }
        }

        this.setState({
          searchOntClassResults,
          searchingOntClasses: false
        });
      });
  };

  private onChangeOntClassValue = (rval: any) => {
    const value = ClassId.deserialize4str(rval.key);
    this.setState({
      classId: { value, ...this.validateClassIdField(value) }
    });
  };

  private validateClassIdField = (val?: ClassId) => {
    if (val !== undefined && val instanceof ClassId) {
      return {
        validateStatus: "success" as "success",
        errorMsg: undefined
      };
    }

    return {
      validateStatus: "error" as "error",
      errorMsg: "Cannot be empty"
    };
  };
}

export default injectStyles(styles)(ClassIdSelector);
