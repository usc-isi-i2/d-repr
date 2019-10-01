import * as constants from "../constants";
import { DReprAction } from "../actions";
import { defaultAppStoreState, AppTbl } from "../types";

export function appReducer(
  state: AppTbl = defaultAppStoreState,
  action: DReprAction
): AppTbl {
  switch (action.type) {
    case constants.APP_RELOAD: {
      return action.app;
    }
    case constants.APP_SET_SYNC: {
      const ns = state.cloneRef();
      ns.synchStatus = action.syncStatus;
      return ns;
    }
    default: {
      return state;
    }
  }
}
