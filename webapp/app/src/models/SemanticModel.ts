import * as _ from "lodash";
import { ClassId } from "./ClassId";
import { Ontology } from ".";

type NodeType = "classnode" | "datanode" | "literalnode";
export type DataType =
  | "xsd:int"
  | "xsd:string"
  | "xsd:decimal"
  | "xsd:dateTime"
  | "xsd:anyURI";
export const DATA_TYPES: DataType[] = [
  "xsd:string",
  "xsd:int",
  "xsd:decimal",
  "xsd:dateTime",
  "xsd:anyURI"
];

export class Node {
  public static classnode(nid: ClassId) {
    return new Node(nid.id, nid.shortURI, "classnode", undefined, nid);
  }

  public static datanode(variableId: string, dataType: DataType) {
    return new Node(variableId, variableId, "datanode", dataType, undefined);
  }

  public static literalnode(nid: string, data: string, dataType: DataType) {
    return new Node(nid, data, "literalnode", dataType, undefined);
  }

  public id: string;
  public label: string;
  public type: NodeType;
  public dataType: DataType | undefined;
  private classId_?: ClassId;

  private constructor(
    id: string,
    label: string,
    type: NodeType,
    dataType?: DataType,
    classId?: ClassId
  ) {
    this.id = id;
    this.label = label;
    this.type = type;
    this.dataType = dataType;
    this.classId_ = classId;
  }

  get classId() {
    return this.classId_;
  }

  public isClassNode() {
    return this.type === "classnode";
  }

  public isDataNode() {
    return this.type === "datanode";
  }
}

class Edge {
  public id: string;
  public sourceId: string;
  public targetId: string;
  public predicate: string;

  constructor(
    id: string,
    sourceId: string,
    predicate: string,
    targetId: string
  ) {
    this.id = id;
    this.predicate = predicate;
    this.sourceId = sourceId;
    this.targetId = targetId;
  }
}

export class SemanticModel {
  public static default() {
    return new SemanticModel(
      {},
      {},
      {},
      { classnode: [], datanode: [] },
      {},
      {}
    );
  }

  public static deserialize(sm: any) {
    const that = SemanticModel.default();
    _.each(sm.data_nodes, (dnode: any, varId: string) => {
      that.addNode(Node.datanode(varId, dnode.data_type));
      if (!that.hasNode(dnode.node_id)) {
        that.addNode(Node.classnode(ClassId.deserialize4str(dnode.node_id)));
      }
      that.addEdge(dnode.node_id, dnode.predicate, varId);
    });
    for (const srel of sm.relations) {
      that.addEdge(srel.source_id, srel.predicate, srel.target_id);
    }

    return that;
  }

  private id2node: { [id: string]: Node };
  private id2edge: { [id: string]: Edge };
  // store nodes in their index ascending order
  private lbl2nodes: { [lbl: string]: Node[] };
  // store nodes in arbitrary order
  private type2nodes: { [type: string]: Node[] };
  // store outgoing links and incoming links of nodes
  private incomingEdges: { [id: string]: Set<string> };
  private outgoingEdges: { [id: string]: Set<string> };

  private constructor(
    id2node: { [id: string]: Node },
    id2edge: { [id: string]: Edge },
    lbl2nodes: { [lbl: string]: Node[] },
    type2nodes: { [type: string]: Node[] },
    incomingEdges: { [id: string]: Set<string> },
    outgoingEdges: { [id: string]: Set<string> }
  ) {
    this.id2edge = id2edge;
    this.id2node = id2node;
    this.lbl2nodes = lbl2nodes;
    this.type2nodes = type2nodes;
    this.incomingEdges = incomingEdges;
    this.outgoingEdges = outgoingEdges;
  }

  public cloneRef() {
    return new SemanticModel(
      this.id2node,
      this.id2edge,
      this.lbl2nodes,
      this.type2nodes,
      this.incomingEdges,
      this.outgoingEdges
    );
  }

  public addNode(n: Node) {
    this.id2node[n.id] = n;
    this.incomingEdges[n.id] = new Set();
    this.outgoingEdges[n.id] = new Set();

    if (!(n.label in this.lbl2nodes)) {
      this.lbl2nodes[n.label] = [];
    }

    if (n.isClassNode()) {
      this.lbl2nodes[n.label].splice(
        _.sortedIndexBy(
          this.lbl2nodes[n.label],
          n,
          (o: Node) => o.classId!.index
        ),
        0,
        n
      );
      this.type2nodes.classnode.push(n);
    } else {
      this.lbl2nodes[n.label].push(n);
      this.type2nodes.datanode.push(n);
    }
  }

  public hasNode(nid: string) {
    return nid in this.id2node;
  }

  public addEdge(sourceId: string, predicate: string, targetId: string) {
    const eid = `${sourceId}---${predicate}---${targetId}`;

    if (!(sourceId in this.id2node) || !(targetId in this.id2node)) {
      throw new Error("Cannot add an edge from un-existing node");
    }

    if (eid in this.id2edge) {
      throw new Error("Cannot have same predicate between two nodes");
    }

    this.id2edge[eid] = new Edge(eid, sourceId, predicate, targetId);
    this.incomingEdges[targetId].add(eid);
    this.outgoingEdges[sourceId].add(eid);
  }

  public removeEdge(eid: string): Edge {
    const e = this.id2edge[eid];
    this.outgoingEdges[e.sourceId].delete(eid);
    this.incomingEdges[e.targetId].delete(eid);
    delete this.id2edge[eid];
    return e;
  }

  public upwardCascadeRemoveEdge(eid: string) {
    const e = this.removeEdge(eid);
    if (this.getNOutgoingEdges(e.sourceId) === 0) {
      // delete class nodes that don't have any children
      this.cascadeRemoveNode(e.sourceId);
    }
  }

  public cascadeRemoveNode(nid: string): void {
    const n = this.id2node[nid];
    const cascadeNodes = [];

    // remove all incoming links
    for (const eid of this.incomingEdges[nid]) {
      const e = this.removeEdge(eid);
      const source = this.id2node[e.sourceId];

      if (source.isClassNode() && this.getNOutgoingEdges(e.sourceId) === 0) {
        cascadeNodes.push(e.sourceId);
      }
    }

    if (n.isClassNode()) {
      // remove all outgoing links
      for (const eid of this.outgoingEdges[nid]) {
        const e = this.removeEdge(eid);
        const target = this.id2node[e.targetId];

        if (target.isClassNode()) {
          // delete a class node when there is no outgoing edges
          if (this.getNOutgoingEdges(e.targetId) === 0) {
            cascadeNodes.push(e.targetId);
          }
        } else {
          // delete a data node when there is no incoming edge
          if (this.getNIncomingEdges(e.targetId) === 0) {
            cascadeNodes.push(e.targetId);
          }
        }
      }

      const idx = this.type2nodes.classnode.indexOf(n);
      this.type2nodes.classnode.splice(idx, 1);
    } else {
      const idx = this.type2nodes.datanode.indexOf(n);
      this.type2nodes.datanode.splice(idx, 1);
    }

    delete this.id2node[nid];
    delete this.outgoingEdges[nid];
    delete this.incomingEdges[nid];
    this.lbl2nodes[n.label].splice(this.lbl2nodes[n.label].indexOf(n), 1);
    if (this.lbl2nodes[n.label].length === 0) {
      delete this.lbl2nodes[n.label];
    }

    for (const cid of cascadeNodes) {
      this.cascadeRemoveNode(cid);
    }
  }

  public *iterNodes(): IterableIterator<Node> {
    for (const nid in this.id2node) {
      yield this.id2node[nid];
    }
  }

  public *iterDataNodes(): IterableIterator<Node> {
    for (const n of this.type2nodes.datanode) {
      yield n;
    }
  }

  public *iterClassNodes(): IterableIterator<Node> {
    for (const n of this.type2nodes.classnode) {
      yield n;
    }
  }

  public *iterTriples(): IterableIterator<[Node, string, Node]> {
    for (const eid in this.id2edge) {
      const e = this.id2edge[eid];
      yield [this.id2node[e.sourceId], e.predicate, this.id2node[e.targetId]];
    }
  }

  public *iterEdges(): IterableIterator<Edge> {
    for (const eid in this.id2edge) {
      yield this.id2edge[eid];
    }
  }

  public *iterIncomingEdges(nid: string): IterableIterator<Edge> {
    for (const eid of this.incomingEdges[nid]) {
      yield this.id2edge[eid];
    }
  }

  public *iterOutgoingEdges(nid: string): IterableIterator<Edge> {
    for (const eid of this.outgoingEdges[nid]) {
      yield this.id2edge[eid];
    }
  }

  public getEdgeById(eid: string): Edge {
    return this.id2edge[eid];
  }

  public getNodeById(nid: string): Node {
    return this.id2node[nid];
  }

  public getDataNodeSemanticType(nid: string) {
    if (!this.hasNode(nid)) {
      return undefined;
    }

    const node = this.id2node[nid];
    if (this.incomingEdges[node.id].size === 0) {
      return undefined;
    }

    const edge = this.id2edge[
      this.incomingEdges[node.id].values().next().value
    ];
    return {
      ontClass: this.id2node[edge.sourceId].classId,
      ontPredicate: edge.predicate
    };
  }

  public getNOutgoingEdges(nid: string) {
    return this.outgoingEdges[nid].size;
  }

  public getNIncomingEdges(nid: string) {
    return this.incomingEdges[nid].size;
  }

  // return list of class nodes by their short URI in ascending order
  public getClassNodesByShortURI(shortURI: string): Node[] {
    return this.lbl2nodes[shortURI] || [];
  }

  public getUsedNamespacePrefixes(): Set<string> {
    const ns = new Set();
    for (const n of this.iterClassNodes()) {
      ns.add(n.classId!.getNamespacePrefix());
    }

    // TODO: fix me, hard code
    for (const e of this.iterEdges()) {
      if (e.predicate.indexOf(":") !== -1) {
        ns.add(e.predicate.split(":", 2)[0]);
      }
    }
    return ns;
  }

  public serialize(onts: { [prefix: string]: Ontology }) {
    const dataNodes = {};
    const relations = [];
    const literalNodes = [];

    for (const e of this.iterEdges()) {
      const target = this.id2node[e.targetId];
      if (target.isClassNode()) {
        relations.push({
          source: e.sourceId,
          predicate: e.targetId,
          target: e.targetId
        });
      } else if (target.isDataNode()) {
        dataNodes[e.targetId] = {
          node_id: e.sourceId,
          class_uri: this.id2node[e.sourceId].label,
          predicate: e.predicate,
          data_type: target.dataType
        };
      } else {
        literalNodes.push({
          node_id: e.sourceId,
          class_uri: this.id2node[e.sourceId].label,
          predicate: e.predicate,
          data: target.label,
          data_type: target.dataType
        });
      }
    }

    const usedNs = this.getUsedNamespacePrefixes();
    return {
      data_nodes: dataNodes,
      literal_nodes: literalNodes,
      relations,
      prefixes: _.fromPairs(_.map([...usedNs], ns => [ns, onts[ns].namespace]))
    };
  }
}
