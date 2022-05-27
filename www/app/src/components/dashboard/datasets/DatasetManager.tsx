import * as React from "react";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import { connect } from "react-redux";
import {
  Row,
  Radio,
  Col,
  Card,
  Button,
  Menu,
  Icon,
  Dropdown,
  Alert,
  Table,
  Input,
  Upload,
  notification
} from "antd";
import { DB, AppTbl, DatasetTbl } from "src/store/types";
import Form, { FormComponentProps } from "antd/lib/form";
import { Dispatch } from "redux";
import {
  datasetList,
  datasetCreate,
  datasetSelect,
  datasetRemove,
  datasetDeSelect
} from "src/store/actions";
import * as _ from "lodash";
import { timingSafeEqual } from "crypto";
import { UploadFile } from "antd/lib/upload/interface";
import { ClickParam } from "antd/lib/menu";

const styles = {};

interface Props extends WithStyles<typeof styles>, FormComponentProps {
  dispatch: Dispatch;
  app: AppTbl;
  datasetTbl: DatasetTbl;
}

// tslint:disable-next-line:no-empty-interface
interface State {}

interface Record {
  name: string;
  description: string;
  isActive: boolean;
}

class DatasetManager extends React.Component<Props, State> {
  public state: State = {};
  private isSelectingDataset: boolean = false;
  private datasetColumns = [
    {
      title: "Name",
      dataIndex: "name"
    },
    {
      title: "Description",
      dataIndex: "description",
      // TODO: fix me! temporary fix for long description
      width: 500
    },
    {
      title: "Action",
      key: "action",
      render: (record: Record) => {
        return (
          <React.Fragment>
            <Button
              type="primary"
              ghost={true}
              onClick={this.toggleDatasetActivation(record.name)}
            >
              {record.isActive ? "deactivate" : "activate"}
            </Button>
            <Button
              type="danger"
              ghost={true}
              className="margin-left-8"
              onClick={this.removeDataset(record.name)}
            >
              Delete
            </Button>
          </React.Fragment>
        );
      }
    }
  ];

  public componentDidMount = () => {
    // send request to server to reload the data
    this.props.dispatch(datasetList());
  };

  public render() {
    const { getFieldDecorator } = this.props.form;
    return (
      <React.Fragment>
        <Row gutter={8}>
          <Col span={24}>
            {this.props.datasetTbl.activeDataset !== null && (
              <Alert
                type="success"
                message={
                  <div style={{ minHeight: 35 }}>
                    <div style={{ float: "right" }}>
                      <div style={{ display: "inline-block" }}>
                        <Upload
                          beforeUpload={this.uploadRepr}
                          showUploadList={false}
                        >
                          <Button>
                            <Icon type="upload" /> Upload D-Repr
                          </Button>
                        </Upload>
                      </div>
                      <Button
                        onClick={this.downloadRepr}
                        className="margin-left-8"
                      >
                        Download D-Repr
                      </Button>
                      <Dropdown
                        overlay={
                          <Menu onClick={this.downloadData}>
                            <Menu.Item key="ttl">Turtle</Menu.Item>
                          </Menu>
                        }
                        className="margin-left-8"
                      >
                        <Button>
                          Download data as <Icon type="down" />
                        </Button>
                      </Dropdown>
                      <Button className="margin-left-8" type="primary" onClick={this.finishModeling}>
                        Finish
                      </Button>
                      <Button className="margin-left-8" type="danger">
                        Remove
                      </Button>
                    </div>
                    <label style={{ fontSize: 16, fontWeight: 600 }}>
                      Active dataset: {this.props.datasetTbl.activeDataset}
                    </label>
                  </div>
                }
              />
            )}
          </Col>
        </Row>
        <Row gutter={8} className="margin-top-8">
          <Col span={24}>
            <Table
              columns={this.datasetColumns}
              dataSource={_.map(this.props.datasetTbl.datasets, d => {
                return {
                  ...d,
                  key: d.name,
                  isActive: d.name === this.props.datasetTbl.activeDataset
                };
              })}
              bordered={true}
            />
          </Col>
        </Row>
        <Row gutter={8}>
          <Col span={24}>
            <Form layout="inline" onSubmit={this.createDataset}>
              <Form.Item label="Name">
                {getFieldDecorator("name", {
                  rules: [
                    { required: true, message: "Please provide a dataset name" }
                  ]
                })(<Input placeholder="name" />)}
              </Form.Item>
              <Form.Item label="Description">
                {getFieldDecorator("description")(
                  <Input placeholder="description" />
                )}
              </Form.Item>
              <Form.Item>
                <Button type="primary" htmlType="submit">
                  Create Dataset
                </Button>
              </Form.Item>
            </Form>
          </Col>
        </Row>
      </React.Fragment>
    );
  }

  private uploadRepr = (file: UploadFile) => {
    const formData = new FormData();
    formData.set("repr_file", file as any);

    this.props.app
      .post(`/datasets/${this.props.datasetTbl.activeDataset}/repr`, formData)
      .then(() => {
        notification.success({
          message: "upload success"
        });

        // TODO: fix me! fetch new dataset
        return this.props.dispatch(
          datasetSelect(this.props.datasetTbl.activeDataset as string)
        );
      })
      .catch(() => {
        notification.error({
          message: "upload failed"
        });
      });

    return false;
  };

  private downloadRepr = () => {
    this.props.app
      .get(`/datasets/${this.props.datasetTbl.activeDataset}/repr`)
      .then(resp => {
        return new Blob([resp.data]);
      })
      .then(blob => {
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.style.display = "none";
        a.href = url;
        // the filename you want
        a.download = `${this.props.datasetTbl.activeDataset}.model.yml`;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
      })
      .catch(() => {
        notification.error({
          message: "Error while downloading a file"
        });
      });
  };

  private downloadData = (params: ClickParam) => {
    if (params.key === "ttl") {
      this.props.app
        .get(`/datasets/${this.props.datasetTbl.activeDataset}/data`)
        .then(resp => {
          return new Blob([resp.data]);
        })
        .then(blob => {
          const url = window.URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.style.display = "none";
          a.href = url;
          // the filename you want
          a.download = `${this.props.datasetTbl.activeDataset}.ttl`;
          document.body.appendChild(a);
          a.click();
          window.URL.revokeObjectURL(url);
        })
        .catch(() => {
          notification.error({
            message: "Error while downloading a file"
          });
        });
    } else {
      notification.error({
        message: "Not Implemented Yet"
      });
    }
  };

  private finishModeling = () => {
    const urlParams = new URLSearchParams(window.location.search);
    const notifyUri = urlParams.get("on_finish");
    if (notifyUri !== null) {
      window.open(notifyUri, "_self");
    }
  };

  private createDataset = (e: any) => {
    e.preventDefault();
    this.props.form.validateFieldsAndScroll((err, values) => {
      if (!err) {
        return this.props
          .dispatch(datasetCreate(values.name, values.description))
          .then(() => {
            this.props.form.resetFields();
          });
      }
    });
  };

  private removeDataset = (datasetName: string) => {
    return () => {
      this.props.dispatch(datasetRemove(datasetName));
    };
  };

  private toggleDatasetActivation = (datasetName: string) => {
    return () => {
      if (this.isSelectingDataset) {
        return;
      }

      this.isSelectingDataset = true;
      if (this.props.datasetTbl.activeDataset === datasetName) {
        this.props.dispatch(datasetDeSelect());
        this.isSelectingDataset = false;
      } else {
        return this.props.dispatch(datasetSelect(datasetName)).then(() => {
          this.isSelectingDataset = false;
        });
      }
    };
  };
}

function db2Props(store: DB) {
  return {
    datasetTbl: store.datasets,
    app: store.app
  };
}

export default Form.create({})(
  connect(db2Props)(injectStyles(styles)(DatasetManager))
);
