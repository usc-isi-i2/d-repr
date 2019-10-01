import * as constants from "../constants";
import { ResourceRecord, DB } from "../types";
import { DataSlice, PortionOfData } from "src/models/data";
import { DReprAction } from ".";
import { Dispatch } from "redux";
import axios from "axios";
import { Alignment } from "src/models";

export interface AlignmentUpsert {
  type: constants.ALIGNMENT_UPSERT;
  mapping: Alignment;
}

export interface MappingRemove {
  type: constants.ALIGNMENT_REMOVE;
  mapping: Alignment;
}

export function alignmentUpsert(alignment: Alignment): AlignmentUpsert {
  return {
    type: constants.ALIGNMENT_UPSERT,
    mapping: alignment
  };
}

export function alignmentRemove(alignment: Alignment): MappingRemove {
  return {
    type: constants.ALIGNMENT_REMOVE,
    mapping: alignment
  };
}
