import * as _ from "lodash";
import { IDGenerator } from "../../misc/IDGenerator";
import { DataSlice, ContinuousRangeDataSlice, IndexDataSlice } from ".";

export abstract class Dimension {
  public static deserialize(
    val: any,
    depth: number = 0,
    idgen: IDGenerator = new IDGenerator()
  ): Dimension {
    if (val.type === "range") {
      if (
        val.range.length !== val.values.length + 1 ||
        val.values.length === 0 ||
        val.range[0] < 0
      ) {
        throw new Error("Invalid values of RangeDimension");
      }

      const range = [];
      for (const v of val.range) {
        if (v === "inf") {
          range.push(Infinity);
          continue;
        } else {
          range.push(v);
        }

        if (typeof v !== "number") {
          throw new Error("Invalid range values of RangeDimension");
        }
      }
      for (let i = 1; i < range.length; i++) {
        if (range[i] < range[i - 1]) {
          throw new Error("Invalid range values of RangeDimension");
        }
      }

      const values = [];
      for (const v of val.values) {
        if (v === null) {
          values.push(v);
        } else {
          values.push(Dimension.deserialize(v, depth + 1, idgen));
        }
      }

      return new RangeDimension(`rd_${idgen.next()}`, depth, range, values);
    }

    if (val.type === "index") {
      const index2val = {};
      for (const k of Object.keys(val.values)) {
        if (val.values[k] === null) {
          index2val[k] = null;
        } else {
          index2val[k] = Dimension.deserialize(val.values[k], depth + 1, idgen);
        }
      }

      return new IndexDimension(`id_${idgen.next()}`, depth, index2val);
    }

    throw new Error(
      `Invalid type of dimension. Expected range or index. Got ${val.type}`
    );
  }

  public readonly id: string; // unique id of the dimension
  public readonly depth: number; // since this is tree structure, depth here is the same concept

  constructor(id: string, depth: number) {
    this.id = id;
    this.depth = depth;
  }

  // return sub-dimension
  public abstract get(idx: string | number): Dimension | null | undefined;
  // return the maximum number of dimensions in this heterogeneous data
  public abstract getMaxNDims(): number;
  // validate if data slices is valid
  public abstract isValidSlice(
    slices: DataSlice,
    allowSlicePassEnd?: boolean
  ): boolean;
  // get the size of the dimension, undefined if the size is varied
  public abstract size(): undefined | number;
  public abstract iterDFS(
    depth?: number
  ): IterableIterator<[Dimension, number]>;
}

export type DimensionImpl = RangeDimension | IndexDimension;

export class RangeDimension extends Dimension {
  // the start is always >= 0, and the end can be a number or Infinity
  public range: number[];
  public values: Array<DimensionImpl | null>;

  constructor(
    id: string,
    depth: number,
    range: number[],
    values: Array<DimensionImpl | null>
  ) {
    super(id, depth);
    this.range = range;
    this.values = values;
  }

  public get(idx: string | number): Dimension | null {
    return this.values[_.sortedLastIndex(this.range, idx as number) - 1];
  }

  public getMaxNDims(): number {
    return (
      Math.max(...this.values.map(v => (v === null ? 0 : v.getMaxNDims()))) + 1
    );
  }

  public isValidSlice(
    slice: DataSlice,
    allowSlicePassEnd: boolean = false
  ): boolean {
    if (slice === null) {
      return false;
    }

    if (slice instanceof ContinuousRangeDataSlice) {
      if (
        slice.start < this.range[0] ||
        (!allowSlicePassEnd &&
          slice.end !== Infinity &&
          slice.end > this.range[this.range.length - 1])
      ) {
        // out of the range
        return false;
      }
      // we accept the empty slice at the end
      if (
        slice.start === slice.end &&
        this.range[this.range.length - 1] === slice.end
      ) {
        return true;
      }

      // we accept a data slice what span through multiple heterogeneous dimensions
      let idx = _.sortedLastIndex(this.range, slice.start) - 1;
      do {
        if (
          !(
            (this.values[idx] === null && slice.values[idx] === null) ||
            (this.values[idx] !== null &&
              this.values[idx]!.isValidSlice(
                slice.values[idx]!,
                allowSlicePassEnd
              ))
          )
        ) {
          return false;
        }
        idx += 1;
      } while (idx < this.values.length && this.range[idx] < slice.end);

      return true;
    }

    return false;
  }

  public size(): undefined | number {
    if (this.range[this.range.length - 1] === Infinity) {
      return undefined;
    }

    return this.range[this.range.length - 1] - this.range[0];
  }

  public *iterDFS(depth: number = 0): IterableIterator<[Dimension, number]> {
    yield [this, depth];
    for (let i = 0; i < this.range.length - 1; i++) {
      const csptr = this.get(this.range[i]);
      if (csptr === null) {
        continue;
      }

      for (const [s, d] of csptr.iterDFS(depth + 1)) {
        yield [s, d];
      }
    }
  }
}

export class IndexDimension extends Dimension {
  public index2val: { [index: string]: DimensionImpl | null };

  constructor(
    id: string,
    depth: number,
    index2val: { [index: string]: DimensionImpl | null }
  ) {
    super(id, depth);
    this.index2val = index2val;
  }

  public get(idx: string | number): Dimension | null | undefined {
    return this.index2val[idx];
  }

  public getMaxNDims(): number {
    return (
      Math.max(
        ...Object.keys(this.index2val).map((k: string) =>
          this.index2val[k] === null
            ? 0
            : (this.index2val[k] as DimensionImpl).getMaxNDims()
        )
      ) + 1
    );
  }

  public isValidSlice(
    slice: DataSlice,
    allowSlicePassEnd: boolean = false
  ): boolean {
    if (slice === null) {
      return false;
    }

    if (slice instanceof IndexDataSlice) {
      for (const k of Object.keys(slice.index2slice)) {
        if (!(k in this.index2val)) {
          return false;
        }

        if (
          !(
            (this.index2val[k] === null && slice.index2slice[k] === null) ||
            this.index2val[k]!.isValidSlice(
              slice.index2slice[k]!,
              allowSlicePassEnd
            )
          )
        ) {
          return false;
        }
      }

      return true;
    }

    return false;
  }

  public size(): undefined | number {
    return Object.keys(this.index2val).length;
  }

  public *iterDFS(depth: number = 0): IterableIterator<[Dimension, number]> {
    yield [this, depth];

    for (const idx in this.index2val) {
      const val = this.index2val[idx];
      if (val === null) {
        continue;
      }

      for (const [s, d] of val.iterDFS(depth + 1)) {
        yield [s, d];
      }
    }
  }
}
