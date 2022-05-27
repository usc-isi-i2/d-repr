import * as constants from "../constants";
import { DReprAction } from "../actions";
import {
  defaultResourceStoreState,
  ResourcesTbl,
  ResourceRecord
} from "../types";
import * as _ from "lodash";

export function resourceReducer(
  state: ResourcesTbl = defaultResourceStoreState,
  action: DReprAction
): ResourcesTbl {
  switch (action.type) {
    case constants.DATASET_SELECT: {
      return _.keyBy(
        action.resources,
        (r: ResourceRecord) => r.resource.resourceId
      );
    }
    case constants.DATASET_DESELECT: {
      return defaultResourceStoreState;
    }
    case constants.RESOURCE_UPSERT: {
      return { ...state, [action.resourceID]: action.resource };
    }
    case constants.RESOURCE_UPDATE_DATA: {
      state[action.resourceID].data.pod = action.pod;
      return { ...state, [action.resourceID]: state[action.resourceID] };
    }
    case constants.RESOURCE_DELETE: {
      const ns = { ...state };
      delete ns[action.resourceId];
      return ns;
    }
    default: {
      return state;
    }
  }
}
