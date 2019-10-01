import * as _ from "lodash";
import { DataSlice } from ".";
import { Dimension } from "./Dimension";

export class RangeUtil<T> {
  public readonly range: number[];
  public readonly values: Array<T | null>;

  // use to manage range & values
  constructor(range: number[], values: Array<T | null>) {
    this.range = range;
    this.values = values;
  }

  public validate(): boolean {
    if (this.range.length !== this.values.length + 1) {
      return false;
    }

    // must be monotonically increasing
    for (let i = 1; i < this.range.length; i++) {
      if (this.range[i] < this.range[i - 1]) {
        return false;
      }
    }

    return true;
  }

  public changeRange(start: number, size: number): RangeUtil<T> {
    // change the range so that range[0] = start & size of range = size
    if (start < this.range[0] || start >= this.range[this.range.length - 1]) {
      throw new Error("Invalid start");
    }

    size = Math.min(size, this.range[this.range.length - 1] - start);

    let idx = _.sortedLastIndex(this.range, start) - 1;
    const range = [start];
    const values = [this.values[idx]];
    while (++idx < this.range.length) {
      if (this.range[idx] - range[0] >= size) {
        range.push(range[0] + size);
        break;
      }

      range.push(this.range[idx]);
      values.push(this.values[idx]);
    }

    return new RangeUtil(range, values);
  }

  public optimize(): RangeUtil<T> {
    const range = [this.range[0]];
    const values = [];

    for (let i = 1; i < this.values.length; i++) {
      if (!_.isEqual(this.values[i - 1], this.values[i])) {
        range.push(this.range[i]);
        values.push(this.values[i - 1]);
      }
    }

    range.push(this.range[this.range.length - 1]);
    values.push(this.values[this.values.length - 1]);

    return new RangeUtil(range, values);
  }
}

export class DataSliceIndex {
  public static getWithoutIndex(dslice: DataSlice, id: string) {
    for (const [stpr, depth] of dslice.iterDFS()) {
      if (stpr.id === id) {
        return stpr;
      }
    }

    throw new Error(`Doesn't have any data slice with id ${id}`);
  }

  private idmap: { [id: string]: DataSlice };

  constructor(dslice: DataSlice) {
    this.idmap = {};
    for (const [stpr, depth] of dslice.iterDFS()) {
      this.idmap[stpr.id] = stpr;
    }
  }

  public get(id: string) {
    return this.idmap[id];
  }
}

export class DimensionIndex {
  public static getWithoutIndex(dimension: Dimension, id: string) {
    for (const [dptr, depth] of dimension.iterDFS()) {
      if (dptr.id === id) {
        return dptr;
      }
    }

    throw new Error(`Doesn't have any dimension with id ${id}`);
  }

  private idmap: { [id: string]: Dimension };

  constructor(dim: Dimension) {
    this.idmap = {};
    for (const [dtpr, depth] of dim.iterDFS()) {
      this.idmap[dtpr.id] = dtpr;
    }
  }

  public get(id: string) {
    return this.idmap[id];
  }
}
