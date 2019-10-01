import { Icon, Layout, Menu } from "antd";
import { SelectParam } from "antd/lib/menu";
import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { AppTbl } from "src/store/types";
import DatasetManager from "./datasets/DatasetManager";
import SimpleOntologyManager from "./ontology/SimpleOntologyManager";
import Settings from "./settings/Settings";

const styles = {
  full: {}
};

const defaultProps = {
  clazz: "full" as string
};

interface Props
  extends WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  app: AppTbl;
}

type MenuItem = "settings" | "datasets" | "ontology";

interface State {
  activeMenu: MenuItem;
}

class ControlPanel extends React.Component<Props, State> {
  public static defaultProps = defaultProps;
  public state: State = {
    activeMenu: "datasets"
  };

  public render() {
    let component;
    // only show user menu when user hasn't logged in
    switch (this.state.activeMenu) {
      case "settings":
        component = <Settings />;
        break;
      case "datasets":
        component = <DatasetManager />;
        break;
      case "ontology":
        component = <SimpleOntologyManager />;
        break;
    }

    return (
      <Layout style={{ padding: "10px 0", background: "#fff", height: "100%" }}>
        <Layout.Sider width={200} style={{ background: "#fff" }}>
          <Menu
            mode="inline"
            onSelect={this.onChangeMenu}
            selectedKeys={[this.state.activeMenu]}
            style={{ height: "100%" }}
          >
            <Menu.Item key="datasets">
              <Icon type="database" />
              Datasets
            </Menu.Item>
            <Menu.Item key="ontology">
              <Icon type="cluster" />
              Ontology
            </Menu.Item>
            <Menu.Item key="settings">
              <Icon type="setting" />
              Settings
            </Menu.Item>
          </Menu>
        </Layout.Sider>
        <Layout.Content style={{ padding: "0 10px" }}>
          {component}
        </Layout.Content>
      </Layout>
    );
  }

  private onChangeMenu = (selection: SelectParam) => {
    this.setState({ activeMenu: selection.key as MenuItem });
  };
}

export default injectStyles(styles)(ControlPanel);
