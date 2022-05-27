import * as React from "react";
import { injectStyles, WithStyles } from "src/misc/JssInjection";
import { Card, Divider, Button } from "antd";
import { Variable } from "src/models";

const styles = {
  root: {
    "&>div": {
      padding: "8px !important"
    },
    "& > div > .variable-list-header": {
      minWidth: 80,
      display: "inline-block",
      fontWeight: 600
    },
    "& .ant-divider": {
      height: "1.5em"
    },
    "& button": {
      marginRight: 8
    }
  }
};

const defaultProps = {
  onVariableClick: (variableId: string) => {
    /* do nothing */
  },
  highlightVariableIds: new Set()
};

interface Props
  extends WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  variables: Variable[];
  selectedVariableId?: string;
  highlightVariableIds: Set<string>;
  onVariableClick: (variableId: string) => void;
}

class VariableList extends React.Component<Props, object> {
  public static defaultProps = defaultProps;

  public render() {
    const variables = [];

    for (const v of this.props.variables) {
      const btnType =
        v.id === this.props.selectedVariableId
          ? "primary"
          : this.props.highlightVariableIds.has(v.id)
          ? "dashed"
          : "default";
      variables.push(
        <Button key={v.id} onClick={this.onVariableClick(v.id)} type={btnType}>
          {v.id}
        </Button>
      );
    }

    return (
      <Card className={this.props.classes.root}>
        <div className="variable-list-header">Attributes</div>
        <Divider type="vertical" />
        {variables}
      </Card>
    );
  }

  private onVariableClick = (variableId: string) => {
    return () => {
      this.props.onVariableClick(variableId);
    };
  };
}

export default injectStyles(styles)(VariableList);
