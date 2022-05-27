import * as constants from "../constants";
import { Dispatch } from "redux";
import { DReprAction } from ".";
import { Ontology } from "src/models";
import * as _ from "lodash";
import { DB } from "../types";
import axios from "axios";
import { notification } from "antd";

export interface OntologyList {
  type: constants.ONTOLOGY_LIST;
  ontologies: Ontology[];
}

export interface OntologyRemove {
  type: constants.ONTOLOGY_REMOVE;
  prefix: string;
  namespace: string;
}

export interface OntologyCreate {
  type: constants.ONTOLOGY_CREATE;
  prefix: string;
  namespace: string;
}

export function ontologyList(): any {
  return async (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const resp = await axios.get("/ontologies");
    dispatch({
      type: constants.ONTOLOGY_LIST,
      ontologies: _.map(
        resp.data.ontologies,
        (uri, ns) => new Ontology(ns, uri)
      )
    });
  };
}

export function ontologyRemove(prefix: string, ns: string): any {
  return async (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    return getState()
      .app.delete(`/ontologies/${ns}`)
      .then(() => {
        return dispatch({
          type: constants.ONTOLOGY_REMOVE,
          prefix,
          namespace: ns
        });
      });
  };
}

export function ontologyCreate(
  ontFile: File,
  prefix: string,
  namespace: string
): any {
  return async (dispatch: Dispatch<DReprAction>, getState: () => DB) => {
    const formData = new FormData();
    formData.set("ontology_file", ontFile);
    formData.set("prefix", prefix);
    formData.set("namespace", namespace);

    return getState()
      .app.post(`/ontologies`, formData, {
        headers: {
          "Content-Type": "multipart/form-data"
        }
      })
      .then((resp: { data: { n_classes: number; n_predicates: number } }) => {
        if (resp.data.n_classes + resp.data.n_predicates === 0) {
          notification.warning({
            message: "ontology",
            description:
              "The ontology is imported, but 0 class and predicate was added"
          });
          return null;
        }

        notification.info({
          message: "Import ontology success",
          description: `${resp.data.n_classes} classes added and ${
            resp.data.n_predicates
          } predicates added`
        });

        return dispatch({
          type: constants.ONTOLOGY_CREATE,
          prefix,
          namespace
        });
      });
  };
}
