import { Button, Card, Form, Icon, Input, Select, Upload } from "antd";
import { FormComponentProps } from "antd/lib/form";
import { UploadFile } from "antd/lib/upload/interface";
import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { Dispatch } from "redux";
import { SUPPORT_RESOURCE_TYPES } from "src/models";
import { resourceCreate } from "src/store/actions";
import { DatasetTbl, DB, ResourcesTbl } from "src/store/types";
import { injectStyles, WithStyles } from "../../misc/JssInjection";

const styles = {
  marginRightButton: {
    marginRight: 8
  },
  createResourceForm: {
    marginTop: 8
  },
  uploadBtnContainer: {
    float: "right" as "right"
  },
  resourceInputContainer: {
    overflow: "hidden",
    paddingRight: 8
  }
};

const formItemProps = {
  labelCol: { span: 4 },
  wrapperCol: { span: 20 }
};
const formTailItemProps = { wrapperCol: { span: 20, offset: 4 } };
const ERROR_MSG_REQUIRE_EITHER_URL_OR_FILE =
  "Must upload a resource or specify its URL";
const defaultProps = {
  onClose: () => {
    /* empty function */
  }
};

interface PublicProps
  extends Readonly<typeof defaultProps>,
    FormComponentProps {}

interface Props extends WithStyles<typeof styles>, PublicProps {
  datasets: DatasetTbl;
  resources: ResourcesTbl;
  dispatch: Dispatch;
}

interface State {
  // need to manage the resoucreFiles as we want to allow maximum one file
  resourceFiles: UploadFile[];
  showExtra: boolean;
}

export class UpsertResourceForm extends React.Component<Props, State> {
  public static defaultProps = defaultProps;
  public state: State = {
    resourceFiles: [],
    showExtra: false
  };

  public render() {
    const { getFieldDecorator } = this.props.form;
    const resourceTypes = _.map(
      SUPPORT_RESOURCE_TYPES,
      (description, resourceType) => (
        <Select.Option
          key={resourceType}
          data-testid="antd-select-options"
          data-testvalue={resourceType}
        >
          {description}
        </Select.Option>
      )
    );
    let resourceIdRules;
    if (_.size(this.props.resources) === 0) {
      resourceIdRules = {
        initialValue: "default",
        rules: [{ required: false, whitespace: true }]
      };
    } else {
      resourceIdRules = {
        rules: [{ required: true, whitespace: true }]
      };
    }

    let showExtraBtn = null;
    const extraProperties = [];

    if (this.props.form.getFieldValue("resourceType")) {
      showExtraBtn = (
        <a
          style={{ marginLeft: 8, fontSize: 12 }}
          onClick={this.toggleShowExtraProperties}
        >
          {this.state.showExtra ? "Collapse" : "Show"} extra properties{" "}
          <Icon type={this.state.showExtra ? "up" : "down"} />
        </a>
      );
    }

    if (this.state.showExtra) {
      switch (this.props.form.getFieldValue("resourceType").label) {
        case SUPPORT_RESOURCE_TYPES.csv: {
          extraProperties.push(
            <Form.Item
              label="Delimiter"
              key="extra-delimiter"
              {...formItemProps}
              data-testid="upsert-resource-form-delimiter-wrapper"
            >
              {getFieldDecorator("delimiter", {
                initialValue: ",",
                rules: [
                  {
                    required: false,
                    whitespace: true,
                    validator: (rule: any, value: any, callback: any) => {
                      if (this.normDelimiterChar(value).length !== 1) {
                        callback([
                          new Error("Delimiter must be 1-character string")
                        ]);
                      }
                    }
                  }
                ]
              })(<Input data-testid="upsert-resource-form-delimiter" />)}
            </Form.Item>
          );
          break;
        }
        case SUPPORT_RESOURCE_TYPES.json:
        case SUPPORT_RESOURCE_TYPES.netcdf4:
          break;
        default: {
          throw new Error(
            `Doesn't handle type ${
              this.props.form.getFieldValue("resourceType").key
            } yet`
          );
        }
      }
    }

    return (
      <Card size="small">
        <h2>Create Resource</h2>
        <Form
          onSubmit={this.createNewResource}
          data-testid="upsert-resource-form"
        >
          <Form.Item label="Id" {...formItemProps}>
            {getFieldDecorator("resourceId", resourceIdRules)(
              <Input data-testid="upsert-resource-form-resource-id" />
            )}
          </Form.Item>
          <Form.Item label="Type" {...formItemProps}>
            {getFieldDecorator("resourceType", {
              rules: [
                { required: true, message: "Please select resource type" }
              ]
            })(
              <Select
                showSearch={true}
                labelInValue={true}
                data-testid="upsert-resource-form-resource-type"
              >
                {resourceTypes}
              </Select>
            )}
          </Form.Item>
          <Form.Item label="Remote File" {...formItemProps}>
            {getFieldDecorator("resourceURL", {
              rules: [
                {
                  whitespace: true
                }
              ]
            })(<Input data-testid="upsert-resource-form-resource-url" />)}
          </Form.Item>
          <Form.Item label="Upload File" {...formItemProps}>
            {getFieldDecorator("resourceFile")(
              <Upload
                onRemove={this.onRemoveResourceFile}
                beforeUpload={this.onSelectResourceFile}
                fileList={this.state.resourceFiles}
              >
                <Button>
                  <Icon type="upload" /> Upload file
                </Button>
              </Upload>
            )}
          </Form.Item>
          {extraProperties}
          <Form.Item {...formTailItemProps}>
            <Button
              className={this.props.classes.marginRightButton}
              type="danger"
            >
              Clear
            </Button>
            <Button
              onClick={this.props.onClose}
              className={this.props.classes.marginRightButton}
            >
              Cancel
            </Button>
            <Button
              type="primary"
              htmlType="submit"
              data-testid="upsert-resource-form-submit-button"
            >
              Create
            </Button>
            {showExtraBtn}
          </Form.Item>
        </Form>
      </Card>
    );
  }

  private toggleShowExtraProperties = () => {
    if (!this.state.showExtra) {
      // only show extra properties when resource type is selected
      if (!this.props.form.getFieldValue("resourceType")) {
        return;
      }
    }
    this.setState({ showExtra: !this.state.showExtra });
  };

  private onRemoveResourceFile = (file: UploadFile) => {
    this.setState({ resourceFiles: [] });
  };

  private onSelectResourceFile = (file: UploadFile) => {
    this.setState({
      resourceFiles: [file]
    });

    // update resource url
    const errors = this.props.form.getFieldError("resourceURL");
    if (errors !== undefined && errors.length > 0) {
      if (
        errors[0].toString().indexOf(ERROR_MSG_REQUIRE_EITHER_URL_OR_FILE) !==
        -1
      ) {
        this.props.form.setFields({
          resourceURL: {
            errors: undefined
          }
        });
      }
    }

    return false;
  };

  private createNewResource = (e: any) => {
    e.preventDefault();
    this.props.form.validateFieldsAndScroll((err, values) => {
      if (!err) {
        try {
          if (values.resourceId in this.props.resources) {
            // duplicated resource id
            this.props.form.setFields({
              resourceId: {
                value: values.resourceId,
                errors: [new Error("Duplicated resource id")]
              }
            });
            return;
          }

          if (
            this.state.resourceFiles.length === 0 &&
            (values.resourceURL === undefined ||
              values.resourceURL.trim() === "")
          ) {
            this.props.form.setFields({
              resourceURL: {
                value: undefined,
                errors: [new Error(ERROR_MSG_REQUIRE_EITHER_URL_OR_FILE)]
              }
            });
            return;
          }

          let resourceLoc;
          let extra;
          if (values.resourceFile) {
            resourceLoc = {
              isFile: true,
              value: this.state.resourceFiles[0] as any
            };
          } else {
            resourceLoc = { isFile: false, value: values.resourceURL };
          }

          switch (values.resourceType.label) {
            case SUPPORT_RESOURCE_TYPES.csv:
              extra = {
                delimiter: this.normDelimiterChar(values.delimiter || ",")
              };
              break;
            case SUPPORT_RESOURCE_TYPES.json:
            case SUPPORT_RESOURCE_TYPES.netcdf4:
              extra = {};
              break;
            default:
              throw new Error(
                `Doesn't handle extra properties of ${
                  values.resourceType.key
                } yet`
              );
          }

          this.props
            .dispatch(
              resourceCreate(
                values.resourceId,
                values.resourceType.key,
                resourceLoc,
                extra
              )
            )
            .then(() => {
              return this.props.onClose();
            })
            .catch((error: any) => {
              // TODO: handle this correctly
              global.console.error("[ERROR]", error);
            });
        } catch (error) {
          global.console.error("[BUG]", error);
        }
      }
    });
  };

  private normDelimiterChar(delimiter: string) {
    switch (delimiter) {
      case "\\t":
        return "\t";
      case "\\n":
        return "\n";
      default:
        return delimiter;
    }
  }
}

function db2props(state: DB) {
  return {
    resources: state.resources,
    app: state.app,
    datasets: state.datasets
  };
}

export default Form.create<PublicProps>({})(
  connect(db2props)(injectStyles(styles)(UpsertResourceForm))
);
