import * as constants from "../constants";
import { DReprAction } from "../actions";
import { VariableTbl, defaultVariableStoreState } from "../types";
import { Variable } from "src/models";
import * as _ from "lodash";

export function variableReducer(
  state: VariableTbl = defaultVariableStoreState,
  action: DReprAction
): VariableTbl {
  switch (action.type) {
    case constants.DATASET_SELECT: {
      return _.keyBy(action.variables, (v: Variable) => v.id);
    }
    case constants.DATASET_DESELECT: {
      return defaultVariableStoreState;
    }
    case constants.VARIABLE_UPSERT: {
      const newState = { ...state };
      if (action.previousVariableId !== null) {
        delete newState[action.previousVariableId];
      }

      newState[action.variable.id] = action.variable;
      return newState;
    }
    case constants.VARIABLE_DELETE: {
      const newState = { ...state };
      delete newState[action.variableId];
      return newState;
    }
    default: {
      return state;
    }
  }
}
