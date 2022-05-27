import * as React from "react";
import { WithStyles } from "@material-ui/core";
import { injectStyles } from "src/misc/JssInjection";
import * as cytoscape from "cytoscape";
import * as _ from "lodash";
import { Divider, Icon, Button } from "antd";
import memoizeOne from "memoize-one";
import edgehandles from "cytoscape-edgehandles";
import spread from "cytoscape-spread";

cytoscape.use(edgehandles);
cytoscape.use(spread);

const styles = {
  root: {
    width: "100%",
    minHeight: "500px",
    border: "1px solid #e8e8e8",
    marginTop: 8,
    "&>div:last-child": {
      width: "100%",
      minHeight: "500px"
    },
    position: "relative" as "relative"
  },
  rootHasTitle: {
    borderTop: "none",
    marginTop: "-28px !important",
    paddingTop: 16
  },
  title: {
    marginTop: "8px !important"
  },
  floatedControlBtn: {
    position: "absolute" as "absolute",
    top: 5,
    right: 0,
    zIndex: 990,
    "& button": {
      marginRight: 4,
      fontSize: "1.2em",
      lineHeight: "1.2em",
      padding: "0 10px 2px",
      fontWeight: 500
    }
  }
};

const defaultEdgeDrawingStyles = [
  {
    selector: ".eh-handle",
    style: {
      "background-color": "#db0d11",
      width: 12,
      height: 12,
      shape: "ellipse",
      "overlay-opacity": 0,
      "border-width": 12, // makes the handle easier to hit
      "border-opacity": 0
    }
  },
  {
    selector: ".eh-source",
    style: {
      "border-width": 2,
      "border-color": "#db0d11"
    }
  },
  {
    selector: ".eh-target",
    style: {
      "border-width": 2,
      "border-color": "#db0d11"
    }
  },
  {
    selector: ".eh-preview, .eh-ghost-edge",
    style: {
      width: 2,
      "background-color": "#db0d11",
      "line-color": "#db0d11",
      "curve-style": "bezier",
      "control-point-step-size": 75,
      "target-arrow-color": "#db0d11",
      "target-arrow-shape": "triangle"
    }
  }
];

const defaultProps = {
  onNodeClick: (nid: string) => {
    /* do nothing */
  },
  onNodeRightClick: (nid: string) => {
    /* do nothing */
  },
  onEdgeClick: (eid: string) => {
    /* do nothing */
  },
  enableEdgeDrawingCreation: false,
  canDrawEdgeBetweenNodes: (sourceId: string, targetId: string) => {
    return true;
  },
  onDrawNewEdgeComplete: (sourceId: string, targetId: string) => {
    /* do nothing */
  }
};

interface Props
  extends WithStyles<typeof styles>,
    Readonly<typeof defaultProps> {
  title?: string;
  nodes: Node[];
  edges: Edge[];
  styles: any;
  layout: any;
}

export interface Node {
  id: string;
  label: string;
  type: string;
}

export interface Edge {
  id?: string;
  label: string;
  source: string;
  target: string;
}

class CytoscapeGraph extends React.Component<Props, object> {
  public static defaultProps = defaultProps;
  private container: React.RefObject<HTMLDivElement>;
  private cy?: cytoscape.Core = undefined;
  private cyEdgeDrawing: {
    eh?: any;
    isHoveringEh: boolean;
    isHoveringNode: boolean;
  } = {
    isHoveringEh: false,
    isHoveringNode: false
  };
  private syncWithCytoscape = memoizeOne(
    (nodes: Node[], edges: Edge[], cy?: cytoscape.Core) =>
      this.syncWithCytoscape_(nodes, edges, cy)
  );

  constructor(props: Props) {
    super(props);
    this.container = React.createRef();
  }

  public componentDidMount() {
    if (this.cy === undefined) {
      this.cy = cytoscape({
        container: this.container.current!,
        layout: {
          name: "breadthfirst",
          fit: true,
          spacingFactor: 1,
          directed: true
        },
        style: this.props.styles.concat(defaultEdgeDrawingStyles)
      });

      // checkout the configuration at: https://github.com/cytoscape/cytoscape.js-edgehandles
      this.cyEdgeDrawing.eh = (this.cy as any).edgehandles({
        preview: false,
        hoverDelay: 150,
        handleNodes: "node",
        snap: false,
        snapThreshold: 50,
        snapFrequency: 15,
        noEdgeEventsInDraw: true,
        disableBrowserGestures: true,
        handleInDrawMode: false,
        nodeLoopOffset: -50,
        edgeType: (sourceNode: any, targetNode: any) => {
          return this.props.canDrawEdgeBetweenNodes(
            sourceNode.id(),
            targetNode.id()
          )
            ? "flat"
            : null;
        },
        edgeParams: (sourceNode: any, targetNode: any, i: any) => {
          return { data: { label: "" } };
        },
        ghostEdgeParams: () => {
          // return element object to be passed to cy.add() for the ghost edge
          // (default classes are always added for you)
          return { data: { label: "" } };
        },
        complete: (sourceNode: any, targetNode: any, addedEles: any) => {
          // fired when edgehandles is done and elements are added
          this.props.onDrawNewEdgeComplete(sourceNode.id(), targetNode.id());
          // remove temporary edges
          addedEles.remove();
        }
      });
      (window as any).eh = this.cyEdgeDrawing.eh;
      // as the cytoscape doesn't hide eh-handle correctly, we have to handle it ourself.
      // when mouse moves out a node, we only hide eh-handle if it's not hovering
      // and when mouse moves out eh-handle, we ohly hide if it's not hovering any node
      // as the order of firing events is not guarantee, we give them some event a bit of delay
      // to make sure they're fired in this order:
      // entering eh-handle -> leaving node -> entering node -> leaving eh-handle
      const minDelay = 50;
      this.cy.on("mouseover", ".eh-handle", () => {
        this.cyEdgeDrawing.isHoveringEh = true;
        // global.console.log("[EH-handle] Entering");
      });
      this.cy.on(
        "mouseover",
        "node",
        _.debounce(() => {
          // global.console.log("[Node] Entering");
          this.cyEdgeDrawing.isHoveringNode = true;
        }, minDelay * 2)
      );
      this.cy.on(
        "mouseout",
        "node",
        _.debounce(() => {
          this.cyEdgeDrawing.isHoveringNode = false;
          if (!this.cyEdgeDrawing.isHoveringEh) {
            this.cyEdgeDrawing.eh.hide();
            //   global.console.log("[Node] Leaving to space => hide Eh-handle");
            // } else {
            //   global.console.log("[Node] Leaving to Eh-Handle");
          }
        }, minDelay)
      );
      this.cy.on(
        "mouseout",
        ".eh-handle",
        _.debounce(() => {
          this.cyEdgeDrawing.isHoveringEh = false;
          if (!this.cyEdgeDrawing.isHoveringNode) {
            this.cyEdgeDrawing.eh.hide();
            //   global.console.log("[EH-Handle] Leaving to space => Hide Eh-handle");
            // } else {
            //   global.console.log("[EH-Handle] Leaving to node");
          }
        }, minDelay * 3)
      );

      if (this.props.enableEdgeDrawingCreation) {
        this.cyEdgeDrawing.eh.enable();
      } else {
        this.cyEdgeDrawing.eh.disable();
      }

      this.syncWithCytoscape(this.props.nodes, this.props.edges, this.cy);
      this.cy.userZoomingEnabled(false);
      this.cy.on("click", "node", this.onNodeClick);
      this.cy.on("click", "edge", this.onEdgeClick);
      this.cy.on("cxttap", "node", this.onNodeRightClick);
    }
  }

  public render() {
    this.syncWithCytoscape(this.props.nodes, this.props.edges, this.cy);

    let titleComp = null;
    let extraClassName = "";

    if (this.props.title !== undefined) {
      titleComp = (
        <Divider orientation="left" className={this.props.classes.title}>
          {this.props.title}
        </Divider>
      );
      extraClassName = " " + this.props.classes.rootHasTitle;
    }

    return (
      <React.Fragment>
        {titleComp}
        <div className={this.props.classes.root + extraClassName}>
          <div className={this.props.classes.floatedControlBtn}>
            <Button onClick={this.zoomIn}>+</Button>
            <Button onClick={this.zoomOut}>-</Button>
            <Button onClick={this.centerViewPort}>&#x233E;</Button>
          </div>
          <div ref={this.container} />
        </div>
      </React.Fragment>
    );
  }

  private onNodeClick = (e: cytoscape.EventObject) => {
    this.props.onNodeClick(e.target.id());
  };

  private onNodeRightClick = (e: cytoscape.EventObject) => {
    this.props.onNodeRightClick(e.target.id());
  };

  private onEdgeClick = (e: cytoscape.EventObject) => {
    this.props.onEdgeClick(e.target.id());
  };

  private zoomIn = () => {
    if (this.cy !== undefined) {
      this.cy.zoom(this.cy.zoom() + 0.5);
      this.cy.center();
    }
  };

  private zoomOut = () => {
    if (this.cy !== undefined) {
      this.cy.zoom(this.cy.zoom() - 0.5);
      this.cy.center();
    }
  };

  private centerViewPort = () => {
    if (this.cy !== undefined) {
      this.cy.center();
    }
  };

  private syncWithCytoscape_(
    nodes: Node[],
    edges: Edge[],
    cy?: cytoscape.Core
  ) {
    if (cy === undefined) {
      return;
    }

    cy.remove(cy.nodes());
    cy.remove(cy.edges());
    cy.add(
      _.map(nodes, (n: Node) => {
        return { group: "nodes", data: n } as any;
      })
    );
    cy.add(
      _.map(edges, (e: Edge) => {
        return { group: "edges", data: e } as any;
      })
    );

    const layout = cy.layout(this.props.layout);
    layout.run();
    cy.reset();
    cy.center();
  }
}

export default injectStyles(styles)(CytoscapeGraph);
