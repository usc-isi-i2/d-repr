import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Form, Input, Button, Icon, Upload, Tag, Row, Col, Table } from "antd";
import { OntologyTbl, DB, AppTbl } from "src/store/types";
import { connect } from "react-redux";
import * as _ from "lodash";
import { Dispatch } from "redux";
import { FormComponentProps } from "antd/lib/form";
import { notification } from "antd";
import { ontologyRemove, ontologyCreate } from "src/store/actions/ontology";

const styles = {
  uploadBtnContainer: {
    // float: "right" as "right"
  },
  inputOntologyURIContainer: {
    overflow: "hidden",
    paddingRight: 8
  }
};

const colors = [
  "magenta",
  "red",
  "volcano",
  "orange",
  "gold",
  "green",
  "cyan",
  "blue",
  "geekblue",
  "purple"
];

interface Props extends WithStyles<typeof styles>, FormComponentProps {
  ontologies: OntologyTbl;
  dispatch: Dispatch;
  app: AppTbl;
}

interface Record {
  key: string;
  color: string;
  prefix: string;
  uri: string;
}

class SimpleOntologyManager extends React.Component<Props, object> {
  private ontTableColumns = [
    {
      title: "Prefix",
      dataIndex: "prefix",
      render: (prefix: string, record: Record) => {
        return <Tag color={record.color}>{prefix}</Tag>;
      }
    },
    {
      title: "URI",
      dataIndex: "uri"
    },
    {
      title: "Action",
      key: "action",
      render: (record: { prefix: string; uri: string }) => (
        <span>
          <a
            href="javascript:;"
            onClick={this.deleteOntology(record.prefix, record.uri)}
          >
            Delete
          </a>
        </span>
      )
    }
  ];

  public render() {
    const onts: Record[] = [];
    const { getFieldDecorator } = this.props.form;
    for (const prefix in this.props.ontologies) {
      onts.push({
        key: prefix,
        color: colors[onts.length % colors.length],
        prefix: this.props.ontologies[prefix].prefix,
        uri: this.props.ontologies[prefix].namespace
      });
    }

    return (
      <React.Fragment>
        <Row gutter={8}>
          <Col span={24}>
            <Table
              columns={this.ontTableColumns}
              dataSource={onts}
              bordered={true}
            />
          </Col>
        </Row>
        <Row gutter={8}>
          <Col span={24}>
            <Form layout="inline" onSubmit={this.createOntology}>
              <Form.Item label="Prefix">
                {getFieldDecorator("prefix", {
                  rules: [
                    { required: true, message: "Please provide a prefix" }
                  ]
                })(<Input placeholder="prefix" />)}
              </Form.Item>
              <Form.Item label="Namespace">
                {getFieldDecorator("namespace")(
                  <Input placeholder="namespace" />
                )}
              </Form.Item>
              <Form.Item label="Upload File">
                {getFieldDecorator("file")(
                  <Upload beforeUpload={this.stopUpload}>
                    <Button>
                      <Icon type="upload" /> Upload file
                    </Button>
                  </Upload>
                )}
              </Form.Item>
              <Form.Item>
                <Button type="primary" htmlType="submit">
                  Import Ontology
                </Button>
              </Form.Item>
            </Form>
          </Col>
        </Row>
      </React.Fragment>
    );
  }

  private deleteOntology = (prefix: string, uri: string) => {
    return () => {
      this.props.dispatch(ontologyRemove(prefix, uri));
    };
  };

  private createOntology = (e: any) => {
    e.preventDefault();
    this.props.form.validateFieldsAndScroll((err, values) => {
      if (!err) {
        return this.props
          .dispatch(
            ontologyCreate(values.file.file, values.prefix, values.namespace)
          )
          .then(() => {
            this.props.form.resetFields();
          });
      }
    });
  };

  private stopUpload = () => {
    return false;
  };
}

function db2Props(store: DB) {
  return {
    ontologies: store.ontologies,
    app: store.app
  };
}

export default Form.create({})(
  connect(db2Props)(injectStyles(styles)(SimpleOntologyManager))
);
