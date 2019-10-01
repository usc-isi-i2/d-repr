import * as constants from "../constants";

export interface UISetDisplayMax1Resource {
  type: constants.UI_SET_DISPLAY_MAX_1_RESOURCE;
  value: boolean;
}

export function uiSetDisplayMax1Resource(
  value: boolean
): UISetDisplayMax1Resource {
  return {
    type: constants.UI_SET_DISPLAY_MAX_1_RESOURCE,
    value
  };
}
