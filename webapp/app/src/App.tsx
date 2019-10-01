import * as React from "react";
import "./App.css";
import "antd/dist/antd.css";

import ResourceUI from "./components/resources";
import { connect } from "react-redux";
import * as actions from "./store/actions";
import { Dispatch } from "redux";
import { ResourcesTbl, DB, AppTbl, SyncStatus } from "./store/types";
import LayoutUI from "./components/layout";
import { MappingUI } from "./components/mappings";

import {
  Tabs,
  Row,
  Col,
  Icon,
  Layout,
  Menu,
  Breadcrumb,
  Avatar,
  Dropdown,
  notification
} from "antd";
import * as _ from "lodash";
import Dashboard from "./components/dashboard/Dashboard";
import { SemanticModelUI } from "./components/semanticModel";
import Login from "./Login";
import { SelectParam } from "antd/lib/menu";

interface Props {
  dispatch: Dispatch;
  app: AppTbl;
  hasActiveDataset: boolean;
  hasResources: boolean;
}

type MenuItem =
  | "resource"
  | "layout"
  | "mapping"
  | "semantic_model"
  | "dashboard";

interface State {
  activeMenu: MenuItem;
  loaded: boolean;
}

class App extends React.Component<Props, State> {
  public state: State = { activeMenu: "dashboard", loaded: false };

  public componentDidMount() {
    this.props.app.attempt2ReLoggedIn().then((app: AppTbl) => {
      if (app.isLoggedIn()) {
        // reload the content
        this.props
          .dispatch(actions.appReload(app))
          .then(() => {
            this.setState({ loaded: true });
          })
          .catch((error: any) => {
            window.console.error(error);
            notification.error({
              message: "Error while loading application",
              description: "Please try again later"
            });
          });
      } else {
        this.setState({ loaded: true });
      }
    });
  }

  public render() {
    if (!this.state.loaded) {
      return (
        <div style={{ textAlign: "center", marginTop: 30 }}>
          <p>Loading</p>
          <Icon type="loading" style={{ fontSize: "3em" }} />
        </div>
      );
    }

    if (!this.props.app.isLoggedIn()) {
      return <Login />;
    }

    let sttTab;
    switch (this.props.app.synchStatus) {
      case SyncStatus.synched: {
        sttTab = (
          <Icon type="check-circle" theme="twoTone" twoToneColor="#52c41a" />
        );
        break;
      }
      case SyncStatus.synching: {
        sttTab = <Icon type="sync" spin={true} />;
        break;
      }
      case SyncStatus.error: {
        sttTab = <Icon type="warning" theme="twoTone" twoToneColor="#f5222d" />;
        break;
      }
      default: {
        throw new Error(`Invalid sync status ${this.props.app.synchStatus}`);
      }
    }
    sttTab = <span>{sttTab}Dashboard</span>;

    let component = null;
    switch (this.state.activeMenu) {
      case "dashboard": {
        component = <Dashboard app={this.props.app} />;
        break;
      }
      case "resource": {
        component = <ResourceUI />;
        break;
      }
      case "layout": {
        component = <LayoutUI />;
        break;
      }
      case "layout": {
        component = <LayoutUI />;
        break;
      }
      case "mapping": {
        component = <MappingUI />;
        break;
      }
      case "semantic_model": {
        component = <SemanticModelUI />;
        break;
      }
    }

    return (
      <Layout className="App">
        <Layout.Header
          style={{ position: "fixed", zIndex: 1, width: "100%", height: 55 }}
        >
          <h1 className="logo">D-Repr</h1>
          <Menu
            theme="dark"
            mode="horizontal"
            onSelect={this.onChangeMenu}
            selectedKeys={[this.state.activeMenu]}
            style={{ lineHeight: "55px" }}
          >
            <Menu.Item key="dashboard">{sttTab}</Menu.Item>
            <Menu.Item key="resource" disabled={!this.props.hasActiveDataset}>
              <Icon type="database" theme="filled" />
              Resources
            </Menu.Item>
            <Menu.Item key="layout" disabled={!this.props.hasResources}>
              <span>
                <Icon type="layout" theme="filled" />
                Attributes
              </span>
            </Menu.Item>
            <Menu.Item key="mapping" disabled={!this.props.hasResources}>
              <span>
                <Icon type="api" theme="filled" />
                Alignments
              </span>
            </Menu.Item>
            <Menu.Item key="semantic_model" disabled={!this.props.hasResources}>
              <span>
                <Icon type="deployment-unit" />
                Semantic Model
              </span>
            </Menu.Item>
            <Menu.SubMenu
              title={
                <Avatar
                  style={{
                    backgroundColor: "#f56a00",
                    verticalAlign: "middle",
                    marginTop: -3
                  }}
                  size="default"
                >
                  T
                </Avatar>
              }
              key="user"
              style={{
                float: "right"
              }}
            >
              <Menu.Item>
                <span>
                  <Icon type="setting" />
                  Settings
                </span>
              </Menu.Item>
              <Menu.Item>
                <span>
                  <Icon type="logout" />
                  Logout
                </span>
              </Menu.Item>
            </Menu.SubMenu>
          </Menu>
        </Layout.Header>
        <Layout.Content style={{ marginTop: 55, padding: 10, height: "100%" }}>
          {component}
        </Layout.Content>
      </Layout>
    );
  }

  private onChangeMenu = (selection: SelectParam) => {
    this.setState({ activeMenu: selection.key as MenuItem });
  };
}

function db2props(state: DB) {
  return {
    app: state.app,
    hasActiveDataset: state.datasets.activeDataset !== null,
    hasResources: _.size(state.resources) > 0
  };
}

export default connect(db2props)(App);
