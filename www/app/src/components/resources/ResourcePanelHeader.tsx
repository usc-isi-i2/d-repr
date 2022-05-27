import * as React from "react";
import { Resource } from "src/models";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import { Row, Col, Icon, Modal } from "antd";

const styles = {
  root: {
    marginTop: -20,
    marginBottom: 20,
    padding: "0 8px",
    "& > .ant-col-20": {
      fontWeight: 500
    },
    "& > div:last-child": {
      textAlign: "right" as "right"
    }
  }
};

interface Props extends WithStyles<typeof styles> {
  resource: Resource;
  onDeleteResource: (resourceId: string) => void;
}

class ResourcePanelHeader extends React.Component<Props, object> {
  public render() {
    return (
      <Row
        className={this.props.classes.root}
        data-testid="resource-panel-header"
        data-testvalue={this.props.resource.resourceId}
      >
        <Col span={20}>
          {this.props.resource.resourceId.toUpperCase()} (
          {this.props.resource.resourceType.toUpperCase()})
        </Col>
        <Col span={4}>
          {/* <Icon
            type="close-circle"
            theme="filled"
            onClick={this.onDeleteResource}
            data-testid="resource-panel-header-close-button"
          /> */}
        </Col>
      </Row>
    );
  }

  private onDeleteResource = () => {
    Modal.confirm({
      title: "Are you sure you want to delete this resource?",
      okText: "Yes",
      okType: "danger",
      cancelText: "No",
      onOk: () => {
        this.props.onDeleteResource(this.props.resource.resourceId);
      }
    });
  };
}

export default injectStyles(styles)(ResourcePanelHeader);
