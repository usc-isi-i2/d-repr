import { DataSlice, Dimension } from "../data";

export class IncompatibleDataSlice extends Error {
  constructor(dim: Dimension, slice: DataSlice) {
    super(`${slice} is not compatible with ${dim}`);
    Object.setPrototypeOf(this, new.target.prototype);
  }
}
