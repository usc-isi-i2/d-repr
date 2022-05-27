import * as React from "react";
import { Alignment, Variable, ValueAlignment } from "src/models";
import { render } from "react-dom";
import { Row, Button, Col, Icon } from "antd";
import { WithStyles, injectStyles } from "src/misc/JssInjection";

const styles = {
  controller: {
    marginTop: -20,
    textAlign: "center" as "center",
    "& button": {
      marginRight: 8,
      paddingTop: 2
    },
    "& button:last-child": {
      marginRight: 0
    }
  }
};

interface Props extends WithStyles<typeof styles> {
  mapping?: Alignment;
  var1?: Variable;
  var2?: Variable;
  onDiscard: () => void;
  onClear: () => void;
  onSave: (vm: Alignment) => Promise<void>;
}

export class ValueMappingForm extends React.Component<Props, object> {
  public render() {
    return (
      <Row gutter={8}>
        <Col span={6} offset={9} className={this.props.classes.controller}>
          <Button
            shape="circle"
            disabled={!this.props.var1 || !this.props.var2}
            onClick={this.props.onClear}
          >
            <Icon type="close-circle" theme="filled" />
          </Button>
          <Button
            shape="circle"
            type="danger"
            disabled={!this.props.var1 || !this.props.var2}
            onClick={this.props.onDiscard}
          >
            <Icon type="delete" theme="filled" />
          </Button>
          <Button
            shape="circle"
            type="primary"
            disabled={!this.props.var1 || !this.props.var2}
            onClick={this.saveMapping}
          >
            <Icon type="save" theme="filled" />
          </Button>
        </Col>
      </Row>
    );
  }

  private saveMapping = () => {
    const vm = new ValueAlignment(this.props.var1!.id, this.props.var2!.id);
    this.props.onSave(vm);
  };
}

export default injectStyles(styles)(ValueMappingForm);
