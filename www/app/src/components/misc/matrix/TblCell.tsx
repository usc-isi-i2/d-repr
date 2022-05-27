import * as React from "react";
import { Index, UnitOfData } from "../../../models/data";
import { WithStyles, injectStyles } from "src/misc/JssInjection";
import { CellViewPortPosition } from "./CellViewportPosition";
import { Icon } from "antd";
import * as _ from "lodash";

const styles = {
  controlPanel: {
    float: "right" as "right"
  },
  root: {
    margin: -16,
    padding: 16,
    position: "relative" as "relative",
    "& > div": {
      display: "none"
    },
    "&:hover": {
      "& > div": {
        display: "inherit"
      }
    }
  },
  default: {},
  disable: {},
  selected: {
    backgroundColor: "#4caf50 !important",
    // boxShadow: "0px 4px 5px -2px rgba(0, 0, 0, 0.2)",
    border: "1px solid #45a248 !important",
    borderTop: "1px solid #45a248 !important",
    borderLeft: "1px solid #45a248 !important"
  },
  rborderBtn: {
    cursor: "pointer",
    position: "absolute" as "absolute",
    top: 0,
    right: 0,
    backgroundColor: "#ebebeb",
    height: "100%",
    paddingTop: 16,
    paddingLeft: 4,
    paddingRight: 2,
    boxShadow: "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)"
  },
  lborderBtn: {
    cursor: "pointer",
    position: "absolute" as "absolute",
    top: 0,
    left: 0,
    backgroundColor: "#ebebeb",
    height: "100%",
    paddingTop: 16,
    paddingLeft: 2,
    paddingRight: 4,
    boxShadow: "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)",
    "& > i": {
      transform: "rotate(180deg)"
    }
  },
  tborderBtn: {
    position: "absolute" as "absolute",
    width: "100%",
    top: -25,
    left: 0,
    paddingBottom: 10,
    "& > div": {
      cursor: "pointer",
      textAlign: "center" as "center",
      backgroundColor: "#ebebeb",
      boxShadow: "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)",
      "& > i": {
        transform: "rotate(-90deg)"
      }
    }
  },
  bborderBtn: {
    position: "absolute" as "absolute",
    width: "100%",
    bottom: -25,
    left: 0,
    paddingTop: 10,
    zIndex: 3,
    "& > div": {
      cursor: "pointer",
      textAlign: "center" as "center",
      backgroundColor: "#ebebeb",
      boxShadow: "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)",
      "& > i": {
        transform: "rotate(90deg)"
      }
    }
  },
  cornerBtn: {
    cursor: "pointer",
    width: 20,
    backgroundColor: "#ebebeb",
    boxShadow: "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)",
    display: "block" as "block",
    textAlign: "center" as "center"
  },
  rcornerBtn: {
    "& > div": {
      marginRight: 23
    },
    "& > b": {
      // corner Btn
      float: "right" as "right"
    }
  },
  lcornerBtn: {
    "& > div": {
      marginLeft: 23
    },
    "& > b": {
      // corner Btn
      float: "left" as "left"
    }
  }
};

type RBorderType = "top" | "bottom";
type CBorderType = "right" | "left";
type BorderType = RBorderType | CBorderType;
type CellStatus = "defalt" | "selected";
const defaultProps = {
  status: "default" as CellStatus
};

interface Props
  extends WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  status: CellStatus;
  style?: any;
  onCellClick: (indice: Index[]) => void;
  className?: string;
  enableSelection: boolean;
  // the position of the two unbounded dimensions in the index of the unit of data
  // use this so that we can select toward the end, dim0 always row and dim1 always column
  unboundedDim0SliceIdx: number;
  unboundedDim1SliceIdx: number;
  value: UnitOfData;
  vp: CellViewPortPosition;
}

const borderBtnClasses = {
  top: "tborderBtn",
  right: "rborderBtn",
  bottom: "bborderBtn",
  left: "lborderBtn"
};
const cornerBtnClasses = { right: "rcornerBtn", left: "lcornerBtn" };

class DataCell extends React.Component<Props, object> {
  public static defaultProps = defaultProps;

  public render() {
    const hasHiddenContent =
      this.props.enableSelection && this.props.vp.isInBorder();
    const select2all = [];
    if (hasHiddenContent) {
      const nBorder =
        (this.props.vp.isInRightBorder() ? 1 : 0) +
        (this.props.vp.isInLeftBorder() ? 1 : 0) +
        (this.props.vp.isInTopBorder() ? 1 : 0) +
        (this.props.vp.isInBottomBorder() ? 1 : 0);

      if (nBorder > 1) {
        /* do nothing */
        const cborder: CBorderType = this.props.vp.isInRightBorder()
          ? "right"
          : "left";

        select2all.push(
          <div
            className={this.props.classes[borderBtnClasses[cborder]]}
            key={cborder}
            onClick={this.onBorderClick(cborder)}
          >
            <Icon type="double-right" />
          </div>
        );

        const rborder: RBorderType = this.props.vp.isInBottomBorder()
          ? "bottom"
          : "top";
        select2all.push(
          <div
            key={rborder}
            className={
              this.props.classes[cornerBtnClasses[cborder]] +
              " " +
              this.props.classes[borderBtnClasses[rborder]]
            }
          >
            <b
              className={this.props.classes.cornerBtn}
              onClick={this.onCornerClick(rborder, cborder)}
            >
              &#x233E;
            </b>
            <div onClick={this.onBorderClick(rborder)}>
              <Icon type="double-right" />
            </div>
          </div>
        );
      } else {
        const border: BorderType = this.props.vp.isInRightBorder()
          ? "right"
          : this.props.vp.isInLeftBorder()
          ? "left"
          : this.props.vp.isInTopBorder()
          ? "top"
          : "bottom";

        if (border === "top" || border === "bottom") {
          select2all.push(
            <div
              className={this.props.classes[borderBtnClasses[border]]}
              key={border}
            >
              <div onClick={this.onBorderClick(border)}>
                <Icon type="double-right" />
              </div>
            </div>
          );
        } else {
          select2all.push(
            <div
              className={this.props.classes[borderBtnClasses[border]]}
              key={border}
              onClick={this.onBorderClick(border)}
            >
              <Icon type="double-right" />
            </div>
          );
        }
      }
    }

    return (
      <td
        onClick={this.onClick}
        className={
          this.props.classes.root +
          " " +
          this.props.classes[this.props.status] +
          " " +
          this.props.className
        }
        style={this.props.style}
      >
        {this.props.children}
        {select2all}
      </td>
    );
  }

  private onBorderClick = (border: BorderType) => {
    const [dimSliceIdx, index] = this.getDimSliceIdxAndIndex(border);

    return (e: any) => {
      e.stopPropagation();
      const indice = this.props.value.indice.slice();
      indice[dimSliceIdx] = index;
      this.props.onCellClick(indice);
    };
  };

  private onCornerClick = (rborder: RBorderType, cborder: CBorderType) => {
    const [dimSliceIdx0, index0] = this.getDimSliceIdxAndIndex(rborder);
    const [dimSliceIdx1, index1] = this.getDimSliceIdxAndIndex(cborder);

    return (e: any) => {
      e.stopPropagation();
      const indice = this.props.value.indice.slice();
      indice[dimSliceIdx0] = index0;
      indice[dimSliceIdx1] = index1;
      this.props.onCellClick(indice);
    };
  };

  private onClick = () => {
    this.props.onCellClick(this.props.value.indice);
  };

  private getDimSliceIdxAndIndex(border: BorderType) {
    switch (border) {
      case "right": {
        return [this.props.unboundedDim1SliceIdx, Infinity];
      }
      case "left": {
        return [this.props.unboundedDim1SliceIdx, 0];
      }
      case "top": {
        return [this.props.unboundedDim0SliceIdx, 0];
      }
      case "bottom": {
        return [this.props.unboundedDim0SliceIdx, Infinity];
      }
    }
  }
}

export default injectStyles(styles)(DataCell);
