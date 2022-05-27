import { ResourceUpsert, ResourceUpdateData, ResourceDelete } from "./resource";
import { UISetDisplayMax1Resource } from "./uiConf";
import { AppReload, AppSetSync } from "./app";
import { VariableUpsert, VariableDelete } from "./variable";
import { OntologyList, OntologyRemove, OntologyCreate } from "./ontology";
import { MappingRemove, AlignmentUpsert } from "./alignment";
import {
  SemanticModelAddDataNode,
  SemanticModelAddEdge,
  SemanticModelRemoveNode,
  SemanticModelUpdateEdge,
  SemanticModelRemoveEdge
} from "./semanticModel";
import {
  DatasetSelect,
  DatasetRemove,
  DatasetList,
  DatasetCreate,
  DatasetDeSelect
} from "./dataset";

export type DReprAction =
  | AppReload
  | AppSetSync
  | ResourceUpsert
  | ResourceUpdateData
  | ResourceDelete
  | VariableUpsert
  | VariableDelete
  | MappingRemove
  | AlignmentUpsert
  | UISetDisplayMax1Resource
  | OntologyList
  | OntologyRemove
  | OntologyCreate
  | DatasetSelect
  | DatasetRemove
  | DatasetList
  | DatasetCreate
  | DatasetDeSelect
  | SemanticModelAddDataNode
  | SemanticModelAddEdge
  | SemanticModelRemoveNode
  | SemanticModelUpdateEdge
  | SemanticModelRemoveEdge;

export { resourceCreate, resourceDelete, resourceFetchData } from "./resource";
export { variableUpsert, variableDelete } from "./variable";
export { appReload, appSetSync, appLogin } from "./app";
export { uiSetDisplayMax1Resource } from "./uiConf";
export {
  smAddDataNode,
  smAddEdge,
  smRemoveEdge,
  smUpdateEdge,
  smRemoveNode
} from "./semanticModel";
export { ontologyList, ontologyRemove, ontologyCreate } from "./ontology";
export { alignmentUpsert, alignmentRemove } from "./alignment";
export {
  datasetList,
  datasetCreate,
  datasetRemove,
  datasetSelect,
  datasetDeSelect
} from "./dataset";
