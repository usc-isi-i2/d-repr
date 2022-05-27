import { combineReducers } from "redux";
import { resourceReducer } from "./resource";
import { DB } from "../types";
import { DReprAction } from "../actions";
import { uiConfReducer } from "./uiConf";
import { variableReducer } from "./variable";
import { appReducer } from "./app";
import { ontologyReducer } from "./ontology";
import { semanticModelReducer } from "./semanticModel";
import { mappingReducer } from "./alignment";
import { datasetReducer } from "./dataset";

const reducer = combineReducers<DB, DReprAction>({
  app: appReducer,
  uiConf: uiConfReducer,
  datasets: datasetReducer,
  resources: resourceReducer,
  variables: variableReducer,
  ontologies: ontologyReducer,
  semanticModel: semanticModelReducer,
  alignments: mappingReducer
});

export default reducer;
