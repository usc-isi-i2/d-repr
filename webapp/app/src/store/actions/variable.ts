import * as constants from "../constants";
import { Variable } from "src/models";
import { Dispatch } from "react";
import { DReprAction } from ".";
import { DB } from "../types";

export interface VariableUpsert {
  type: constants.VARIABLE_UPSERT;
  variable: Variable;
  previousVariableId: string | null;
}

export interface VariableDelete {
  type: constants.VARIABLE_DELETE;
  variableId: string;
}

export function variableUpsert(
  variable: Variable,
  previousVariableId: string | null = null
): VariableUpsert {
  return {
    type: constants.VARIABLE_UPSERT,
    variable,
    previousVariableId
  };
}

export function variableDelete(variableId: string): VariableDelete {
  return {
    type: constants.VARIABLE_DELETE,
    variableId
  };
}
