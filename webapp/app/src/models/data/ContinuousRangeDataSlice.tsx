import {
  DataSlice,
  Index,
  Dimension,
  RangeDimension,
  PortionOfData,
  UnitOfData
} from ".";
import * as _ from "lodash";
import { IndexDataSlice } from "./IndexDataSlice";

type PUData = PortionOfData | UnitOfData;
type II<X> = IterableIterator<X>;

export class ContinuousRangeDataSlice extends DataSlice {
  // the start is always >= 0 and the end can be a number or Infinity
  public range: number[];
  public values: Array<DataSlice | null>;

  constructor(id: string, range: number[], values: Array<DataSlice | null>) {
    super(id);
    this.range = range;
    this.values = values;
  }

  get start(): number {
    return this.range[0];
  }

  get end(): number {
    return this.range[this.range.length - 1];
  }

  public isSelected(idx: Index): boolean {
    return this.start <= idx && idx < this.end;
  }

  public setValue(idx: Index, val: DataSlice | null) {
    this.values[_.sortedLastIndex(this.range, idx as number) - 1] = val;
  }

  public getValue(idx: Index) {
    return this.values[_.sortedLastIndex(this.range, idx as number) - 1];
  }

  public isSlicingAllIndices(dim: Dimension): boolean {
    if (dim instanceof RangeDimension) {
      return (
        this.start === dim.range[0] &&
        this.end === dim.range[dim.range.length - 1]
      );
    }

    throw new Error("Invalid type of dim, expected to be RangeDimension");
  }

  public isSelectOne(): boolean {
    return this.end - this.start === 1;
  }

  public toIndex(): Index {
    if (this.isSelectOne()) {
      return this.start;
    }
    throw new Error(
      "Cannot convert the current slice into an index because it selects more than one element"
    );
  }

  public *iter(data: PortionOfData): II<PortionOfData | UnitOfData> {
    if (this.end === Infinity) {
      let idx = this.start;
      while (data.mayHas(idx)) {
        yield data.get(idx);
        idx++;
      }
    } else {
      for (let i = this.start; i < this.end; i++) {
        yield data.get(i);
      }
    }
  }

  public *iterKeys(data: PortionOfData): II<Index> {
    if (this.end === Infinity) {
      let idx = this.start;
      while (data.mayHas(idx)) {
        yield idx;
        idx++;
      }
    } else {
      for (let i = this.start; i < this.end; i++) {
        yield i;
      }
    }
  }

  public *iterEntries(data: PortionOfData): II<[Index, PUData]> {
    if (this.end === Infinity) {
      let idx = this.start;
      while (data.mayHas(idx)) {
        yield [idx, data.get(idx)];
        idx++;
      }
    } else {
      for (let i = this.start; i < this.end; i++) {
        yield [i, data.get(i)];
      }
    }
  }

  public *iterDFS(depth: number = 0): II<[DataSlice, number]> {
    yield [this, depth];
    for (let i = 0; i < this.range.length - 1; i++) {
      const csptr = this.getValue(this.range[i]);
      if (csptr === null) {
        continue;
      }

      for (const [s, d] of csptr.iterDFS(depth + 1)) {
        yield [s, d];
      }
    }
  }

  public *iterDim(dim: Dimension): II<[DataSlice, Dimension]> {
    yield [this, dim];
    for (let i = 0; i < this.range.length - 1; i++) {
      const csptr = this.getValue(this.range[i]);
      if (csptr === null) {
        continue;
      }

      for (const [s, d] of csptr.iterDim(dim.get(this.range[i])!)) {
        yield [s, d];
      }
    }
  }

  public serialize(): object {
    return {
      type: "range",
      range: this.range,
      values: _.map(this.values, v => (v === null ? null : v.serialize()))
    };
  }

  public clone(): DataSlice {
    return new ContinuousRangeDataSlice(
      this.id,
      this.range,
      _.map(this.values, v => (v === null ? null : v.clone()))
    );
  }
}
