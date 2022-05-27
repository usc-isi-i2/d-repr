import axios from "axios";
import * as _ from "lodash";
import { Dispatch } from "redux";
import { DReprAction } from ".";
import { Ontology } from "../../models";
import * as constants from "../constants";
import { AppTbl, DB, SyncStatus } from "../types";
import { datasetSelect } from "./dataset";

export interface AppReload {
  type: constants.APP_RELOAD;
  app: AppTbl;
}

export interface AppSetSync {
  type: constants.APP_SET_SYNC;
  syncStatus: SyncStatus;
}

export function appSetSync(syncStatus: SyncStatus) {
  return {
    type: constants.APP_SET_SYNC,
    syncStatus
  };
}

export function appReload(app: AppTbl): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    return app.get(`/ontologies`).then((resp: any) => {
      dispatch({
        type: constants.APP_RELOAD,
        app
      });

      dispatch({
        type: constants.ONTOLOGY_LIST,
        ontologies: _.map(
          resp.data.ontologies,
          (ns, prefix) => new Ontology(prefix, ns)
        )
      });

      // checking if we need to load specific dataset that user has specified
      const params = new URLSearchParams(window.location.search);
      const datasetName = params.get("dataset");
      if (datasetName !== null) {
        return dispatch(datasetSelect(datasetName, true)).catch(
          (error: any) => {
            if (error.response.status === 404) {
              // the dataset doesn't exist, we silent the error;
            } else {
              throw error;
            }
          }
        );
      }
    });
  };
}

export function appLogin(email: string, password: string): any {
  return (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    return axios
      .post("/login", {
        email,
        password
      })
      .then((resp: any) => {
        // set authentication token
        const newApp = getState().app.cloneRef();
        newApp.setAccessToken(email, resp.data.auth_token);

        return dispatch(appReload(newApp));
      });
  };
}
