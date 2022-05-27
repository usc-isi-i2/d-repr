import * as React from "react";

import { connect } from "react-redux";
import { Dispatch } from "redux";
import { DB, AppTbl, SyncStatus } from "./store/types";
import { WithStyles, injectStyles } from "src/misc/JssInjection";

import { Row, Col, Icon, Form, Button, Checkbox, Input, Alert } from "antd";
import * as _ from "lodash";
import { FormComponentProps } from "antd/lib/form";
import { appLogin } from "./store/actions";

const styles = {
  loginForm: {
    marginTop: 50,
    "& .login-form": {
      marginTop: 10
    }
  },
  loginFormBtn: {
    width: "100%"
  }
};

interface Props extends WithStyles<typeof styles>, FormComponentProps {
  app: AppTbl;
  dispatch: Dispatch;
}

interface State {
  errorMessage: string;
}

class Login extends React.Component<Props, State> {
  public state: State = {
    errorMessage: ""
  };

  public render() {
    const { getFieldDecorator } = this.props.form;

    return (
      <Row gutter={8} className={this.props.classes.loginForm}>
        <Col span={8} offset={8}>
          {this.state.errorMessage.length > 0 && (
            <Alert
              message={this.state.errorMessage}
              type="error"
              banner={true}
              closable={true}
            />
          )}
          <Form
            onSubmit={this.onLogin}
            className="login-form"
            data-testid="user-login-form"
          >
            <Form.Item>
              {getFieldDecorator("email", {
                rules: [{ required: true, message: "Please input your Email!" }]
              })(
                <Input
                  prefix={
                    <Icon type="user" style={{ color: "rgba(0,0,0,.25)" }} />
                  }
                  placeholder="Email"
                  data-testid="user-login-form-email"
                />
              )}
            </Form.Item>
            <Form.Item>
              {getFieldDecorator("password", {
                rules: [
                  { required: true, message: "Please input your Password!" }
                ]
              })(
                <Input
                  prefix={
                    <Icon type="lock" style={{ color: "rgba(0,0,0,.25)" }} />
                  }
                  type="password"
                  placeholder="Password"
                  data-testid="user-login-form-password"
                />
              )}
            </Form.Item>
            <Form.Item>
              {getFieldDecorator("remember", {
                valuePropName: "checked",
                initialValue: true
              })(<Checkbox>Remember me</Checkbox>)}
              {/* <a className="login-form-forgot" href="">
                Forgot password
              </a> */}
              <Button
                type="primary"
                htmlType="submit"
                className={this.props.classes.loginFormBtn}
                // className="login-form-button margin-left-8"
                data-testid="user-login-form-login-button"
              >
                Log in
              </Button>
            </Form.Item>
          </Form>
        </Col>
      </Row>
    );
  }

  private onLogin = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    this.props.form.validateFields((err, values) => {
      if (!err) {
        this.props
          .dispatch(appLogin(values.email, values.password))
          .then(() => {
            this.setState({ errorMessage: "" });
          })
          .catch(() => {
            this.setState({ errorMessage: "Invalid email or password" });
          });
      }
    });
  };
}

function db2props(state: DB) {
  return {
    app: state.app
  };
}

export default Form.create({})(connect(db2props)(injectStyles(styles)(Login)));
