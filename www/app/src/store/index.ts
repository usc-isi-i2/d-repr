import { applyMiddleware, createStore, Dispatch } from "redux";
import reducer from "./reducers";
import { DB } from "./types";
import { DReprAction } from "./actions";
import thunk from "redux-thunk";
import logger from "redux-logger";
import { syncWithServerMiddleware } from "./middlewares";

const store = createStore<DB, any, any, any>(
  reducer,
  applyMiddleware(thunk, logger, syncWithServerMiddleware as any)
);

export type DreprDispatch = Dispatch<DReprAction>;
export default store;
