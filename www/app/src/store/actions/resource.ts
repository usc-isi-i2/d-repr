import * as constants from "../constants";
import { ResourceRecord, DB } from "../types";
import {
  DataSlice,
  PortionOfData,
  Dimension,
  RangeDimension,
  DiscontinuousArrayPortionOfData,
  HashMapPortionOfData
} from "src/models/data";
import { DReprAction } from ".";
import { Dispatch } from "redux";
import { Resource, data as DataModel } from "src/models";

export interface ResourceUpsert {
  type: constants.RESOURCE_UPSERT;
  resourceID: string;
  resource: ResourceRecord;
}

export interface ResourceUpdateData {
  type: constants.RESOURCE_UPDATE_DATA;
  resourceID: string;
  pod: PortionOfData;
}

export interface ResourceDelete {
  type: constants.RESOURCE_DELETE;
  resourceId: string;
}

export function resourceCreate(
  resourceId: string,
  resourceType: string,
  resourceLoc: { isFile: boolean; value: string },
  extra: object
): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const db = getState();
    const formData = new FormData();

    formData.set("resource_id", resourceId);
    formData.set("resource_type", resourceType);

    if (resourceLoc.isFile) {
      formData.set("resource_file", resourceLoc.value);
    } else {
      formData.set("resource_url", resourceLoc.value);
    }
    formData.set("extra", JSON.stringify(extra));

    return db.app
      .post(`/datasets/${db.datasets.activeDataset}/resources`, formData, {
        headers: {
          "Content-Type": "multipart/form-data"
        }
      })
      .then((resp: any) => {
        const resource = resp.data.resource;
        const dimensions = Dimension.deserialize(resource.dimension);
        const pod =
          dimensions instanceof RangeDimension
            ? new DiscontinuousArrayPortionOfData([])
            : new HashMapPortionOfData();

        const record = new ResourceRecord(
          new Resource(resource.id, resource.type),
          new DataModel.NDimData(resource.id, dimensions, pod)
        );

        return dispatch({
          type: constants.RESOURCE_UPSERT,
          resourceID: resourceId,
          resource: record
        });
      });
  };
}

export function resourceDelete(resourceId: string): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const db = getState();
    return db.app
      .delete(`/datasets/${db.datasets.activeDataset}/resources/${resourceId}`)
      .then((resp: any) => {
        dispatch({
          type: constants.RESOURCE_DELETE,
          resourceId
        });
      });
  };
}

function resourceUpdateData(
  resourceID: string,
  pod: PortionOfData
): ResourceUpdateData {
  return {
    type: constants.RESOURCE_UPDATE_DATA,
    resourceID,
    pod
  };
}

export function resourceFetchData(resourceID: string, dslice: DataSlice): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const db = getState();
    return db.app
      .get(`/datasets/${db.datasets.activeDataset}/resources/${resourceID}`, {
        params: { slices: dslice.serialize() }
      })
      .then((resp: any) => {
        const state = getState();
        const data = state.resources[resourceID].data;
        const dataSlice = DataSlice.deserialize(resp.data.slice);
        let action;

        if (data.pod.portionSize() === 0) {
          action = resourceUpdateData(
            resourceID,
            PortionOfData.fromSlices(dslice, data.dimension, resp.data.data)
          );
        } else {
          action = resourceUpdateData(
            resourceID,
            data.pod.addData(dslice, data.dimension, resp.data.data)
          );
        }
        dispatch(action);

        return dataSlice;
      });
  };
}
