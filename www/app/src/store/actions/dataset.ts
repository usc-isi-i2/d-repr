import * as constants from "../constants";
import axios from "axios";
import {
  data,
  Resource,
  Variable,
  Location,
  SemanticModel,
  Ontology,
  Alignment
} from "../../models";
import { ResourceRecord, DB, AppTbl, SyncStatus, Dataset } from "../types";
import { Dispatch } from "redux";
import {
  DiscontinuousArrayPortionOfData,
  HashMapPortionOfData
} from "../../models/data/PortionOfData";
import { DReprAction } from ".";
import { RangeDimension, Dimension } from "src/models/data";
import * as _ from "lodash";

export interface DatasetRemove {
  type: constants.DATASET_REMOVE;
  datasetName: string;
}

export interface DatasetList {
  type: constants.DATASET_LIST;
  datasets: Dataset[];
}

export interface DatasetCreate {
  type: constants.DATASET_CREATE;
  dataset: Dataset;
}

export interface DatasetSelect {
  type: constants.DATASET_SELECT;
  datasetName: string;
  variables: Variable[];
  alignments: Alignment[];
  resources: ResourceRecord[];
  semanticModel: SemanticModel;
}

export interface DatasetDeSelect {
  type: constants.DATASET_DESELECT;
}

export function datasetList(): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    return getState()
      .app.get("/datasets")
      .then((resp: any) => {
        const dsets = _.map(resp.data.datasets, d => ({
          name: d.name,
          description: d.description,
          mbsize: null
        }));

        return dispatch({
          type: constants.DATASET_LIST,
          datasets: dsets
        });
      });
  };
}

export function datasetCreate(name: string, description: string): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    return getState()
      .app.post(`/datasets`, {
        name,
        description
      })
      .then(() => {
        return dispatch({
          type: constants.DATASET_CREATE,
          dataset: {
            name,
            description,
            mbsize: null
          }
        });
      });
  };
}

export function datasetRemove(datasetName: string): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const db = getState();
    if (db.datasets.activeDataset === datasetName) {
      dispatch(datasetDeSelect());
    }

    return db.app.delete(`/datasets/${datasetName}`).then((resp: any) => {
      return dispatch({
        type: constants.DATASET_REMOVE,
        datasetName
      });
    });
  };
}

export function datasetDeSelect(): DatasetDeSelect {
  return {
    type: constants.DATASET_DESELECT
  };
}

export function datasetSelect(
  datasetName: string,
  dismissNotifyError?: boolean
): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const db = getState();
    return db.app
      .get(`/datasets/${datasetName}`, undefined, dismissNotifyError)
      .then((resp: any) => {
        const resources: { [rid: string]: ResourceRecord } = {};
        const variables: Variable[] = [];
        const alignments: Alignment[] = _.map(resp.data.alignments, m =>
          Alignment.deserialize(m)
        );

        for (const rid in resp.data.resources) {
          const rawResource = resp.data.resources[rid];
          const dimension = Dimension.deserialize(rawResource.dimension);
          const pod =
            dimension instanceof RangeDimension
              ? new DiscontinuousArrayPortionOfData([])
              : new HashMapPortionOfData();

          const resource = new ResourceRecord(
            new Resource(rid, rawResource.type),
            new data.NDimData(rid, dimension, pod)
          );

          resources[rid] = resource;
        }

        for (const vid in resp.data.variables) {
          const v = resp.data.variables[vid];
          variables.push(
            new Variable(
              v.id,
              v.sorted,
              v.value_type,
              v.unique,
              v.missing_values,
              Location.fromSerializedSlices(
                v.location.resource_id,
                v.location.slices
              )
            )
          );
        }

        dispatch({
          type: constants.DATASET_SELECT,
          datasetName,
          resources: _.values(resources),
          variables,
          alignments,
          semanticModel: SemanticModel.deserialize(resp.data.semantic_model)
        });
      });
  };
}
