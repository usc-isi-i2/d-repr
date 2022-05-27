import * as constants from "../constants";
import { DReprAction } from "../actions";
import { defaultUIConfiguration, UIConfiguration } from "../types";

export function uiConfReducer(
  state: UIConfiguration = defaultUIConfiguration,
  action: DReprAction
): UIConfiguration {
  switch (action.type) {
    case constants.UI_SET_DISPLAY_MAX_1_RESOURCE:
      return state.setDisplayMax1Resource(action.value);
    default: {
      return state;
    }
  }
}
