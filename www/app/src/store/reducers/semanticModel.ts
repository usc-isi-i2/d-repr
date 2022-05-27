import * as constants from "../constants";
import { DReprAction } from "../actions";
import { defaultAppStoreState, AppTbl, defaultSemanticModel } from "../types";
import { SemanticModel, ClassId, Node } from "src/models";

function addEdge(
  sm: SemanticModel,
  sourceId: ClassId,
  predicate: string,
  targetId: ClassId | string
) {
  if (!sm.hasNode(sourceId.id)) {
    sm.addNode(Node.classnode(sourceId));
  }

  if (targetId instanceof ClassId && !sm.hasNode(targetId.id)) {
    sm.addNode(Node.classnode(targetId));
  }

  if (targetId instanceof ClassId) {
    // add normal edge
    sm.addEdge(sourceId.id, predicate, targetId.id);
  } else {
    // replace edge as data node has maximum one parent
    for (const e of sm.iterIncomingEdges(targetId)) {
      sm.upwardCascadeRemoveEdge(e.id);
    }

    sm.addEdge(sourceId.id, predicate, targetId);
  }
}

export function semanticModelReducer(
  sm: SemanticModel = defaultSemanticModel,
  action: DReprAction
): SemanticModel {
  switch (action.type) {
    case constants.DATASET_SELECT: {
      return action.semanticModel;
    }
    case constants.DATASET_DESELECT: {
      return defaultSemanticModel;
    }
    case constants.SM_ADD_DATA_NODE: {
      if (sm.hasNode(action.dnode.id)) {
        return sm;
      }

      sm.addNode(action.dnode);
      return sm.cloneRef();
    }
    case constants.SM_ADD_EDGE: {
      addEdge(sm, action.sourceId, action.predicate, action.targetId);
      return sm.cloneRef();
    }
    case constants.SM_REMOVE_NODE: {
      sm.cascadeRemoveNode(action.nodeId);
      return sm.cloneRef();
    }
    case constants.SM_REMOVE_EDGE: {
      // also remove parent nodes, we don't remove children node because
      // there may be lots of nodes attached to it
      sm.upwardCascadeRemoveEdge(action.edgeId);
      return sm.cloneRef();
    }
    case constants.SM_UPDATE_EDGE: {
      const edge = sm.getEdgeById(action.edgeId);
      if (
        edge.sourceId === action.sourceId.id &&
        edge.targetId ===
          (action.targetId instanceof ClassId
            ? action.targetId.id
            : action.targetId)
      ) {
        edge.predicate = action.predicate;
      } else {
        addEdge(sm, action.sourceId, action.predicate, action.targetId);

        sm.upwardCascadeRemoveEdge(action.edgeId);
      }
      return sm.cloneRef();
    }
    default:
      return sm;
  }
}
