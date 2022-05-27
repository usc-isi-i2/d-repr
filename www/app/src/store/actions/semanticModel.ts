import * as constants from "../constants";
import { Dispatch } from "redux";
import { DReprAction } from ".";
import { Node } from "src/models";
import * as _ from "lodash";
import { DB } from "../types";
import axios from "axios";
import { ClassId } from "src/models";

export interface SemanticModelAddDataNode {
  type: constants.SM_ADD_DATA_NODE;
  dnode: Node;
}

export interface SemanticModelAddEdge {
  type: constants.SM_ADD_EDGE;
  sourceId: ClassId;
  targetId: ClassId | string;
  predicate: string;
}

export interface SemanticModelRemoveNode {
  type: constants.SM_REMOVE_NODE;
  nodeId: string;
}

export interface SemanticModelRemoveEdge {
  type: constants.SM_REMOVE_EDGE;
  edgeId: string;
}

export interface SemanticModelUpdateEdge {
  type: constants.SM_UPDATE_EDGE;
  edgeId: string;
  sourceId: ClassId;
  targetId: ClassId | string;
  predicate: string;
}

export function smAddDataNode(dnode: Node): SemanticModelAddDataNode {
  if (dnode.isClassNode()) {
    throw new Error("Cannot add ClassNode using AddDataNode function");
  }

  return {
    type: constants.SM_ADD_DATA_NODE,
    dnode
  };
}

export function smAddEdge(
  sourceId: ClassId,
  predicate: string,
  targetId: ClassId | string
): SemanticModelAddEdge {
  return {
    type: constants.SM_ADD_EDGE,
    sourceId,
    targetId,
    predicate
  };
}

export function smRemoveNode(nodeId: string): SemanticModelRemoveNode {
  return { type: constants.SM_REMOVE_NODE, nodeId };
}

export function smRemoveEdge(edgeId: string): SemanticModelRemoveEdge {
  return { type: constants.SM_REMOVE_EDGE, edgeId };
}

export function smUpdateEdge(
  edgeId: string,
  sourceId: ClassId,
  predicate: string,
  targetId: ClassId | string
) {
  return {
    type: constants.SM_UPDATE_EDGE,
    edgeId,
    sourceId,
    targetId,
    predicate
  };
}
