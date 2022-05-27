import { DReprAction, appSetSync } from "../actions";
import { Dispatch, Store } from "redux";
import { DB, SyncStatus } from "../types";
import axios from "axios";
import * as constants from "../constants";
import * as _ from "lodash";

const syncWithServerMiddleware = (store: Store<DB, any>) => (
  next: Dispatch
) => (action: DReprAction) => {
  const result = next(action);

  // check the action type and sync it with the server
  switch (action.type) {
    case constants.VARIABLE_UPSERT: {
      const db = store.getState();
      next(appSetSync(SyncStatus.synching));
      db.app
        .post(`/datasets/${db.datasets.activeDataset}/variables`, {
          prev_id: action.previousVariableId,
          id: action.variable.id,
          sorted: action.variable.sorted,
          value_type: action.variable.type,
          unique: action.variable.unique,
          missing_values: action.variable.missingValues,
          location: {
            resource_id: action.variable.location.resourceId,
            slices: _.map(action.variable.location.slices, s => s.serialize())
          }
        })
        .then((resp: any) => {
          next(appSetSync(SyncStatus.synched));
        })
        .catch(() => {
          next(appSetSync(SyncStatus.error));
        });
      break;
    }
    case constants.VARIABLE_DELETE: {
      const db = store.getState();
      next(appSetSync(SyncStatus.synching));
      db.app
        .delete(
          `/datasets/${db.datasets.activeDataset}/variables/${
            action.variableId
          }`
        )
        .then((resp: any) => {
          next(appSetSync(SyncStatus.synched));
        })
        .catch(() => {
          next(appSetSync(SyncStatus.error));
        });
      break;
    }
    case constants.SM_ADD_DATA_NODE:
      // add a node doesn't actually add new edge, so the model doesn't change => don't need to sync with the server
      break;
    case constants.SM_ADD_EDGE:
    case constants.SM_REMOVE_NODE:
    case constants.SM_REMOVE_EDGE:
    case constants.SM_UPDATE_EDGE: {
      const db = store.getState();
      next(appSetSync(SyncStatus.synching));
      db.app
        .post(
          `/datasets/${db.datasets.activeDataset}/semantic_model`,
          db.semanticModel.serialize(db.ontologies)
        )
        .then((resp: any) => {
          next(appSetSync(SyncStatus.synched));
        })
        .catch(() => {
          next(appSetSync(SyncStatus.error));
        });
      break;
    }
    case constants.ALIGNMENT_REMOVE:
    case constants.ALIGNMENT_UPSERT: {
      const db = store.getState();
      next(appSetSync(SyncStatus.synching));
      db.app
        .post(
          `/datasets/${db.datasets.activeDataset}/alignments`,
          _.map(db.alignments, m => m.serialize())
        )
        .then((resp: any) => {
          next(appSetSync(SyncStatus.synched));
        })
        .catch(() => {
          next(appSetSync(SyncStatus.error));
        });
    }
  }

  return result;
};

export { syncWithServerMiddleware };
