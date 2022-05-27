import UIConfiguration from "./UIConfiguration";
import { ResourceRecord } from "./Resource";
import { Variable, Ontology, SemanticModel, Alignment } from "src/models";
import { AppTbl } from "./AppTbl";
export { AppTbl, SyncStatus } from "./AppTbl";
export { ResourceRecord } from "./Resource";
export { default as UIConfiguration } from "./UIConfiguration";

export interface Dataset {
  name: string;
  description: string;
  mbsize: number | null; // size of the dataset in megabytes
}

export interface DatasetTbl {
  datasets: Dataset[];
  activeDataset: string | null;
}

export interface ResourcesTbl {
  [rid: string]: ResourceRecord;
}

export interface VariableTbl {
  [vid: string]: Variable;
}

export interface OntologyTbl {
  [prefix: string]: Ontology;
}

export type AlignmentTbl = Alignment[];

export interface DB {
  app: AppTbl;
  datasets: DatasetTbl;
  resources: ResourcesTbl;
  variables: VariableTbl;
  ontologies: OntologyTbl;
  alignments: AlignmentTbl;
  semanticModel: SemanticModel;
  uiConf: UIConfiguration;
}

export const defaultResourceStoreState = {};
export const defaultVariableStoreState = {};
export const defaultOntologyStoreState = {};
export const defaultAlignmentStoreState = [];
export const defaultUIConfiguration = new UIConfiguration(true);
export const defaultSemanticModel = SemanticModel.default();
export const defaultAppStoreState = AppTbl.default();
export const defaultDatasetStoreState = { datasets: [], activeDataset: null };
