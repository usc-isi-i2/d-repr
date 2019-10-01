import * as React from "react";
import { ClassId, SemanticModel } from "src/models";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Select, Spin, Form } from "antd";
import axios from "axios";
import * as _ from "lodash";

export interface StringField {
  value?: string;
  validateStatus?: "error" | "success";
  errorMsg?: string;
}

const defaultProps = {
  style: {},
  formItemProps: {}
};

interface Props {
  style: any;
  formItemProps: any;
  fieldName: string;
  value?: string;
  search: (value: string) => Promise<string[]>;
}

interface State {
  searchResults: string[];
  isSearching: boolean;
  field: StringField;
}

export class RStringSelector extends React.Component<Props, State> {
  public static defaultProps = defaultProps;

  constructor(props: Props) {
    super(props);
    this.onSearch = _.debounce(this.onSearch, 200);

    this.state = {
      searchResults: [],
      isSearching: false,
      field: { value: props.value }
    };
  }

  public validate = (): boolean => {
    if (this.state.field.validateStatus === undefined) {
      const field = {
        value: this.state.field.value,
        ...this.validateNotEmptyField(this.state.field.value)
      };

      this.setState({ field });
      return field.validateStatus === "success";
    }
    return this.state.field.validateStatus === "success";
  };

  public getValue = () => {
    return this.state.field.value;
  };

  public render() {
    const searchResults = this.state.searchResults.map(o => (
      <Select.Option key={o}>{o}</Select.Option>
    ));

    return (
      <Form.Item
        label={this.props.fieldName}
        validateStatus={this.state.field.validateStatus}
        help={this.state.field.errorMsg}
        {...this.props.formItemProps}
      >
        <Select
          showSearch={true}
          value={this.state.field.value}
          defaultActiveFirstOption={false}
          notFoundContent={
            this.state.isSearching ? (
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
          onSearch={this.onSearch}
          onChange={this.onChangeValue}
        >
          {searchResults}
        </Select>
      </Form.Item>
    );
  }

  private onSearch = (value: string) => {
    this.setState({ isSearching: true });
    this.props.search(value).then((searchResults: string[]) => {
      this.setState({
        searchResults,
        isSearching: false
      });
    });
  };

  private onChangeValue = (value: string) => {
    this.setState({
      field: { value, ...this.validateNotEmptyField(value) }
    });
  };

  private validateNotEmptyField = (val?: string) => {
    if (val !== undefined && val.length > 0) {
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

export class OntClassSelector extends RStringSelector {
  public static defaultProps = {
    search: (query: string) => {
      return axios
        .get("/ontologies/search", {
          params: { a: "class", q: query }
        })
        .then((resp: any) => resp.data.results);
    },
    ...defaultProps
  };
}

export class OntPredicateSelector extends RStringSelector {
  public static defaultProps = {
    search: (query: string) => {
      return axios
        .get("/ontologies/search", {
          params: { a: "property", q: query }
        })
        .then((resp: any) => resp.data.results);
    },
    ...defaultProps
  };
}
