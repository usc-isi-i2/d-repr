import { Index, Dimension, RangeDimension } from "./data";
import * as _ from "lodash";

export const SLICE_UNSET = -Infinity;
const RANGE_SLICE_REGEX = /^(?:(\d+|(?:\${[^}]+}))?\.\.(\d+|(?:\${[^}]+}))?(?:;(\d+|(?:\${[^}]+})))?)|(\d+|(?:\${[^}]+}))$/;

export class RangeSlice {
  public static default() {
    return new RangeSlice(SLICE_UNSET, SLICE_UNSET, 1);
  }

  public start: number;
  public end: number;
  public step: number;

  constructor(start: number, end: number, step: number = 1) {
    this.start = start;
    this.end = end;
    this.step = step;
  }

  // test if this slice is select only one element
  public isSelectOne(): boolean {
    return (this.end - this.start) / this.step === 1;
  }

  public isSelected(val: number): boolean {
    return (
      val >= this.start &&
      val < this.end &&
      Number.isInteger((val - this.start) / this.step)
    );
  }

  public isRangeOverlapped(start: number, end: number): boolean {
    return end > this.start && this.end > start;
  }

  public clone() {
    return new RangeSlice(this.start, this.end, this.step);
  }

  public isUninitialized() {
    return this.start === SLICE_UNSET && this.end === SLICE_UNSET;
  }

  public toString(): string {
    if (this.isUninitialized()) {
      return " ";
    }

    if (this.end === this.start + this.step) {
      return `${this.start}`;
    }

    const step = this.step === 1 ? "" : `;${this.step}`;
    if (this.end === Infinity) {
      return `${this.start}..${step}`;
    }
    return `${this.start}..${this.end}${step}`;
  }

  public serialize() {
    return {
      type: "range",
      start: this.start,
      end: this.end,
      step: this.step
    };
  }
}

export class IndexSlice {
  public static default() {
    return new IndexSlice(SLICE_UNSET);
  }

  public index: Index;

  constructor(index: Index) {
    this.index = index;
  }

  // test if this slice is select only one element
  public isSelectOne(): boolean {
    return true;
  }

  public isSelected(val: Index): boolean {
    return this.index !== SLICE_UNSET && this.index === val;
  }

  public clone(): IndexSlice {
    return new IndexSlice(this.index);
  }

  public isUninitialized() {
    return this.index === SLICE_UNSET;
  }

  public toString(): string {
    if (this.isUninitialized()) {
      return " ";
    }

    return `${this.index}`;
  }

  public serialize() {
    return {
      type: "index",
      idx: this.index
    };
  }
}

export type Slice = RangeSlice | IndexSlice;

export class Location {
  public static fromSerializedSlices(resourceId: string, slices: any[]) {
    return new Location(
      resourceId,
      slices.map(s => {
        switch (s.type) {
          case "range":
            return new RangeSlice(
              s.start,
              s.end === null ? Infinity : s.end,
              s.step
            );
          case "index":
            return new IndexSlice(s.idx);
          default:
            throw new Error(`Invalid slice type: ${s.type}`);
        }
      })
    );
  }

  public static fromString(resourceId: string, layout: string): Location {
    const slices = [];
    for (const sers of layout.split(":")) {
      const match = RANGE_SLICE_REGEX.exec(sers);
      if (match === null) {
        slices.push(new IndexSlice(sers));
        continue;
      }

      if (match[4] !== undefined) {
        slices.push(new IndexSlice(parseInt(match[4] as string, 10)));
      } else {
        slices.push(
          new RangeSlice(
            parseInt(match[1] || "0", 10),
            match[2] ? parseInt(match[2], 10) : Infinity,
            parseInt(match[3] || "1", 10)
          )
        );
      }
    }

    return new Location(resourceId, slices);
  }

  public resourceId: string;
  public slices: Slice[];

  constructor(resourceId: string, slices: Slice[]) {
    this.resourceId = resourceId;
    this.slices = slices;
  }

  public toString(withoutResourceID: boolean) {
    const layout: string[] = [];
    for (const slice of this.slices) {
      layout.push(slice.toString());
    }

    if (withoutResourceID) {
      return layout.join(":");
    }

    return `${this.resourceId}@${layout.join(":")}`;
  }

  // test if the layout is compatible with the dimension of data source
  public validate(dim: Dimension): boolean {
    return validateSlices(0, this.slices, dim);
  }

  public clone() {
    return new Location(this.resourceId, this.slices.map(s => s.clone()));
  }
}

function validateSlices(
  sliceIdx: number,
  slices: Slice[],
  dim: Dimension
): boolean {
  if (sliceIdx === slices.length) {
    // get to this when haven't consume all dimension is false
    return false;
    // // doing a check here to make sure it is a list, cannot be object
    // // all values need to be null
    // return (
    //   dim instanceof RangeDimension && _.every(dim.values, v => v === null)
    // );
  }

  const slice = slices[sliceIdx];
  if (slice instanceof RangeSlice) {
    if (dim instanceof RangeDimension) {
      if (
        !(
          dim.range[0] <= slice.start &&
          (slice.end === Infinity ||
            dim.range[dim.range.length - 1] >= slice.end)
        )
      ) {
        // some part of slice range is out of dimension range
        return false;
      }

      const startIndex = _.sortedLastIndex(dim.range, slice.start) - 1;
      const subdims = [];
      for (let i = startIndex; i < dim.range.length - 1; i++) {
        subdims.push(dim.values[i]);
        if (dim.range[i + 1] >= slice.end) {
          break;
        }
      }

      for (const sdim of subdims) {
        if (sdim === null) {
          return sliceIdx === slices.length - 1;
        } else if (sdim === undefined) {
          return false;
        }

        if (!validateSlices(sliceIdx + 1, slices, sdim)) {
          return false;
        }
      }

      return true;
    } else {
      return false;
    }
  } else {
    if (dim instanceof RangeDimension && typeof slice.index !== "number") {
      return false;
    }

    const subdim = dim.get(slice.index);
    if (subdim === null) {
      return sliceIdx === slices.length - 1;
    } else if (subdim === undefined) {
      return false;
    }
    return validateSlices(sliceIdx + 1, slices, subdim);
  }
}

export type TypeVariableSorted = "ascending" | "descending" | "none";
export type TypeVariableType =
  | "unspecified"
  | "string"
  | "int"
  | "float"
  | "list[int]"
  | "list[string]"
  | "list[float]";

export class Variable {
  public static default(id: string, location: Location): Variable {
    return new Variable(id, "none", "unspecified", false, [], location);
  }

  public id: string;
  public sorted: TypeVariableSorted;
  public type: TypeVariableType;
  public unique: boolean;
  public missingValues: string[];
  public location: Location;

  constructor(
    id: string,
    sorted: TypeVariableSorted,
    type: TypeVariableType,
    unique: boolean,
    missingValues: string[],
    location: Location
  ) {
    this.id = id;
    this.location = location;
    this.sorted = sorted;
    this.type = type;
    this.unique = unique;
    this.missingValues = missingValues;
  }

  public clone() {
    return new Variable(
      this.id,
      this.sorted,
      this.type,
      this.unique,
      this.missingValues,
      this.location.clone()
    );
  }
}
