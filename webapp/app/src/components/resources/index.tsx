import * as React from "react";
import { injectStyles, WithStyles } from "../../misc/JssInjection";
import UpsertResourceForm from "./UpsertResourceForm";
import { connect } from "react-redux";
import { ResourcesTbl, DB } from "../../store/types";
import { Dispatch } from "redux";
import { Row, Col, Button } from "antd";
import ResourcePanels from "./ResourcePanels";

const styles = {
  createResourceForm: {
    marginBottom: 10
  }
};

const defaultProps = {};

interface Props {
  resources: ResourcesTbl;
}

interface AProps
  extends Props,
    WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  dispatch: Dispatch;
}

interface State {
  displayingCreateResourceForm: boolean;
}

export class ResourceUI extends React.Component<AProps, State> {
  public static defaultProps = defaultProps;
  public state = {
    displayingCreateResourceForm: false
  };

  public displayCreateResourceForm = (event: object) => {
    this.setState({ displayingCreateResourceForm: true });
  };

  public hideCreateResourceForm = () => {
    this.setState({ displayingCreateResourceForm: false });
  };

  public render() {
    let createResourceComponent;
    if (this.state.displayingCreateResourceForm) {
      createResourceComponent = (
        <UpsertResourceForm onClose={this.hideCreateResourceForm} />
      );
    } else {
      createResourceComponent = (
        <Button
          type="primary"
          onClick={this.displayCreateResourceForm}
          data-testid="create-resource-button"
        >
          Create resource
        </Button>
      );
    }

    return (
      <div data-testid="resource-page">
        <Row className={this.props.classes.createResourceForm}>
          <Col span={24}>{createResourceComponent}</Col>
        </Row>
        <ResourcePanels />
      </div>
    );
  }
}

function mapStateToProps(store: DB) {
  return {
    resources: store.resources
  };
}

export default connect(mapStateToProps)(injectStyles(styles)(ResourceUI));
