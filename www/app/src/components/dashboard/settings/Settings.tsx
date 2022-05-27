import * as React from "react";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import { UIConfiguration, DB } from "src/store/types";
import { connect } from "react-redux";
import { Form, Switch } from "antd";

const styles = {};

const defaultProps = {};

interface Props
  extends WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  uiConf: UIConfiguration;
}

const formItemLayout = {
  labelCol: {
    xs: { span: 24 },
    sm: { span: 6 }
  },
  wrapperCol: {
    xs: { span: 24 },
    sm: { span: 18 }
  }
};

class Setting extends React.Component<Props, object> {
  public static defaultProps = defaultProps;

  public render() {
    return (
      <Form>
        <Form.Item label="Display maximum one resource" {...formItemLayout}>
          <Switch checked={this.props.uiConf.displayMax1Resource} />
        </Form.Item>
      </Form>
    );
  }
}

function mapDBToProps(store: DB) {
  return {
    uiConf: store.uiConf
  };
}

export default connect(mapDBToProps)(injectStyles(styles)(Setting));
