import * as constants from "../constants";
import { DReprAction } from "../actions";
import { OntologyTbl, defaultOntologyStoreState } from "../types";
import * as _ from "lodash";
import { Ontology } from "src/models";

export function ontologyReducer(
  state: OntologyTbl = defaultOntologyStoreState,
  action: DReprAction
): OntologyTbl {
  switch (action.type) {
    case constants.ONTOLOGY_LIST: {
      return _.keyBy(action.ontologies, o => o.prefix);
    }
    case constants.ONTOLOGY_CREATE: {
      return {
        ...state,
        [action.prefix]: new Ontology(action.prefix, action.namespace)
      };
    }
    case constants.ONTOLOGY_REMOVE: {
      const newState = { ...state };
      delete newState[action.prefix];
      return newState;
    }
    default:
      return state;
  }
}
