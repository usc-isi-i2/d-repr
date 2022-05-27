import * as constants from "../constants";
import { DReprAction } from "../actions";
import { DatasetTbl, defaultDatasetStoreState } from "../types";
import * as _ from "lodash";

export function datasetReducer(
  state: DatasetTbl = defaultDatasetStoreState,
  action: DReprAction
): DatasetTbl {
  switch (action.type) {
    case constants.DATASET_LIST: {
      // if activeDataset is out of sync, detect it outside of this function
      return {
        datasets: action.datasets,
        activeDataset: state.activeDataset
      };
    }
    case constants.DATASET_DESELECT: {
      return {
        datasets: state.datasets,
        activeDataset: null
      };
    }
    case constants.DATASET_CREATE: {
      return {
        datasets: [...state.datasets, action.dataset],
        activeDataset: state.activeDataset
      };
    }
    case constants.DATASET_SELECT: {
      return {
        datasets: state.datasets,
        activeDataset: action.datasetName
      };
    }
    case constants.DATASET_REMOVE: {
      return {
        datasets: _.filter(state.datasets, d => d.name !== action.datasetName),
        activeDataset:
          state.activeDataset === action.datasetName
            ? null
            : state.activeDataset
      };
    }
    default: {
      return state;
    }
  }
}
