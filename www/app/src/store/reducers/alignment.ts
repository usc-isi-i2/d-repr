import * as constants from "../constants";
import { DReprAction } from "../actions";
import { Alignment } from "../../models";
import { defaultAlignmentStoreState, AlignmentTbl } from "../types";
import * as _ from "lodash";

export function mappingReducer(
  state: AlignmentTbl = defaultAlignmentStoreState,
  action: DReprAction
): AlignmentTbl {
  switch (action.type) {
    case constants.DATASET_SELECT: {
      return action.alignments;
    }
    case constants.DATASET_DESELECT: {
      return defaultAlignmentStoreState;
    }
    case constants.ALIGNMENT_UPSERT: {
      const maps = state.filter(m => !m.isEqual(action.mapping));
      maps.push(action.mapping);
      return maps;
    }
    case constants.VARIABLE_UPSERT: {
      if (action.previousVariableId !== null) {
        const alignments = [];
        for (const align of state) {
          if (align.source === action.previousVariableId) {
            align.source = action.variable.id;
          }
          if (align.target === action.previousVariableId) {
            align.target = action.variable.id;
          }
          alignments.push(align);
        }
        return alignments;
      }
      return state;
    }
    case constants.VARIABLE_DELETE: {
      return state.filter((align: Alignment) => {
        return (
          align.source !== action.variableId &&
          align.target !== action.variableId
        );
      });
    }
    case constants.ALIGNMENT_REMOVE: {
      return state.filter(m => !m.isEqual(action.mapping));
    }
    default: {
      return state;
    }
  }
}
