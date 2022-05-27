import {
  DataSlice,
  Index,
  DataSliceImpl,
  UnitOfData,
  Dimension,
  RangeDimension,
  IndexDimension,
  ContinuousRangeDataSlice,
  IndexDataSlice
} from ".";

export abstract class PortionOfData {
  public static fromSlices(
    slice: DataSlice,
    dim: Dimension,
    data: any
  ): PortionOfData {
    if (!dim.isValidSlice(slice)) {
      throw new Error("The slice doesn't match with the dimension");
    }

    return fromDataSlice(
      slice as DataSliceImpl,
      dim,
      data,
      []
    ) as PortionOfData;
  }

  // If this portion of data contains all indices, it doesn't mean all the data has been fetched
  private $hasAllIndices: boolean = false;
  private $hasAllUpperIndices: boolean = false;

  get hasAllIndices() {
    return this.$hasAllIndices;
  }

  get hasAllUpperIndices() {
    return this.$hasAllUpperIndices;
  }

  public setHasAllIndices() {
    this.$hasAllIndices = true;
    this.$hasAllUpperIndices = true;
  }

  public setHasAllUpperIndices() {
    this.$hasAllUpperIndices = true;
  }

  public abstract get(idx: Index): PortionOfData | UnitOfData;
  // whether it contains the data in this index, always true if we don't have all indices
  public abstract mayHas(idx: Index): boolean;
  public abstract portionSize(): number;
  // convert the data into JSON
  public abstract toJSON(): any;

  public addData(slice: DataSlice, dim: Dimension, data: any): PortionOfData {
    if (!dim.isValidSlice(slice, true)) {
      throw new Error("The slice doesn't match with the dimension");
    }
    return addDataSlice(
      (this as unknown) as PortionOfDataImpl,
      slice as DataSliceImpl,
      dim,
      data,
      []
    ) as PortionOfData;
  }

  public getData(indice: Index[]): UnitOfData {
    if (indice.length === 1) {
      return this.get(indice[0]) as UnitOfData;
    }

    let result: PortionOfData = this;
    for (let i = 0; i < indice.length - 1; i++) {
      result = result.get(indice[i]) as PortionOfData;
    }

    return result.get(indice[indice.length - 1]) as UnitOfData;
  }
}

export type PortionOfDataImpl =
  | HashMapPortionOfData
  | ArrayPortionOfData
  | DiscontinuousArrayPortionOfData;

export class HashMapPortionOfData extends PortionOfData {
  public hashMap: { [value: string]: PortionOfData | UnitOfData };

  public portionSize(): number {
    return Object.keys(this.hashMap).length;
  }

  public mayHas(idx: string): boolean {
    // we say this index exists even when we don't have all data
    return idx in this.hashMap || !this.hasAllIndices;
  }

  public get(idx: string): PortionOfData | UnitOfData {
    return this.hashMap[idx];
  }

  public toJSON(): any {
    const obj = {};
    for (const k of Object.keys(this.hashMap)) {
      const v = this.hashMap[k];
      obj[k] = v instanceof PortionOfData ? v.toJSON() : v.value;
    }
    return obj;
  }
}

export class ArrayPortionOfData extends PortionOfData {
  public start: number;
  public end: number;
  public items: Array<PortionOfData | UnitOfData>;

  constructor(start: number) {
    super();
    this.start = start;
    this.end = start;
    this.items = new Array<PortionOfData | UnitOfData>();
  }

  public mayHas(idx: Index): boolean {
    return this.start <= idx && idx < this.end;
  }

  public portionSize(): number {
    return this.items.length;
  }

  public push(val: PortionOfData | UnitOfData) {
    this.items.push(val);
    this.end++;
  }

  public get(idx: Index): PortionOfData | UnitOfData {
    return this.items[(idx as number) - this.start];
  }

  public set(idx: Index, val: PortionOfData | UnitOfData) {
    this.items[(idx as number) - this.start] = val;
  }

  public toJSON(): any {
    if (this.start !== 0) {
      throw new Error(
        "[BUG] the start should always be 0 because it is not discontinuous"
      );
    }

    return this.items.map(v =>
      v instanceof PortionOfData ? v.toJSON() : v.value
    );
  }
}

export class DiscontinuousArrayPortionOfData extends PortionOfData {
  public arrays: ArrayPortionOfData[];

  constructor(arrays: ArrayPortionOfData[]) {
    super();
    this.arrays = arrays;
  }

  public mayHas(idx: number): boolean {
    for (const item of this.arrays) {
      if (item.mayHas(idx)) {
        return true;
      }
    }

    return !this.hasAllIndices;
  }

  public get(idx: number): PortionOfData | UnitOfData {
    // TODO: optimize me!
    for (const item of this.arrays) {
      if (item.start <= idx && idx <= item.end) {
        return item.get(idx);
      }
    }

    throw Error("Invalid index " + idx);
  }

  public portionSize(): number {
    return this.arrays.reduce((accum, a) => accum + a.portionSize(), 0);
  }

  public toJSON(): any {
    const obj = [];
    for (const darray of this.arrays) {
      while (obj.length < darray.start) {
        obj.push(null);
      }
      for (const v of darray.items) {
        obj.push(v instanceof PortionOfData ? v.toJSON() : v.value);
      }
    }

    return obj;
  }
}

function fromDataSlice(
  slice: DataSliceImpl | null,
  dim: Dimension | null,
  data: any,
  indices: Index[]
): PortionOfData | UnitOfData {
  // TODO: optimize me!
  if (slice === null) {
    // no more element left, you get into the value
    return new UnitOfData(indices.slice(), data);
  }

  if (slice instanceof ContinuousRangeDataSlice) {
    const arrayPortion = new ArrayPortionOfData(slice.start);
    const i = indices.length;
    indices.push(0);

    for (let idx = 0; idx < data.length && idx < slice.end; idx++) {
      indices[i] = slice.start + idx;
      arrayPortion.push(
        fromDataSlice(
          slice.getValue(indices[i]) as DataSliceImpl | null,
          dim!.get(indices[i]) as (Dimension | null),
          data[idx],
          indices
        )
      );
    }

    indices.pop();

    if (slice.isSlicingAllIndices(dim!)) {
      arrayPortion.setHasAllIndices();
      return arrayPortion;
    }

    return new DiscontinuousArrayPortionOfData([arrayPortion]);
  } else {
    const mapPortion = new HashMapPortionOfData();
    const i = indices.length;
    indices.push("");

    for (const idx of Object.keys(slice.index2slice)) {
      indices[i] = idx;
      mapPortion.hashMap[idx] = fromDataSlice(
        slice.index2slice[idx] as DataSliceImpl | null,
        dim!.get(idx) as (Dimension | null),
        data[idx],
        indices
      );
    }

    indices.pop();

    if (slice.isSlicingAllIndices(dim!)) {
      mapPortion.setHasAllIndices();
    }
    return mapPortion;
  }
}

function addDataSlice(
  pod: PortionOfDataImpl,
  slice: DataSliceImpl | null,
  dim: Dimension | null,
  data: any,
  indices: Index[]
): PortionOfData | UnitOfData {
  if (slice === null) {
    // no more elment left, you get into the value
    return new UnitOfData(indices.slice(), data);
  }

  if (pod instanceof HashMapPortionOfData) {
    if (!(slice instanceof IndexDataSlice)) {
      throw new Error(
        "You can only set data for HashMapPortionOfData when using IndexDataSlice"
      );
    }

    const i = indices.length;
    indices.push("");

    for (const k of Object.keys(slice.index2slice)) {
      indices[i] = k;

      if (k in pod.hashMap) {
        addDataSlice(
          pod.hashMap[k] as PortionOfDataImpl,
          slice.index2slice[k] as DataSliceImpl | null,
          dim!.get(k) as (Dimension | null),
          data[k],
          indices
        );
      } else {
        pod.hashMap[k] = fromDataSlice(
          slice.index2slice[k] as DataSliceImpl | null,
          dim!.get(k) as (Dimension | null),
          data[k],
          indices
        );
      }
    }
    indices.pop();

    if (!pod.hasAllIndices) {
      if (
        new Set(Object.keys(pod.hashMap)) ===
        new Set(Object.keys((dim as IndexDimension).index2val))
      ) {
        pod.setHasAllIndices();
      }
    }
    return pod;
  } else {
    if (!(slice instanceof ContinuousRangeDataSlice)) {
      throw new Error(
        "You can only set data for ArrayPortionOfData when using ContinuousRangeDataSlice"
      );
    }

    if (pod instanceof DiscontinuousArrayPortionOfData) {
      let startExtractIndex = -1;
      let endExtractIndex = -1;

      for (const [i, da] of pod.arrays.entries()) {
        if (
          isRangeOverlappedOrContinuous(
            da.start,
            da.end,
            slice.start,
            slice.end
          )
        ) {
          if (startExtractIndex === -1) {
            startExtractIndex = i;
            endExtractIndex = i + 1;
          } else {
            endExtractIndex++;
          }
        }
      }

      if (startExtractIndex === endExtractIndex) {
        // no overlapping or continous
        if (data.length > 0) {
          const darray = fromDataSlice(slice, dim, data, indices);
          if (!(darray instanceof DiscontinuousArrayPortionOfData)) {
            throw new Error(
              "[BUG] Congrat! You found a bug, darray must always be DiscontinuousArrayPOD"
            );
          }

          if (pod.arrays[0].start > darray.arrays[0].start) {
            pod.arrays.unshift(darray.arrays[0]);
          } else {
            pod.arrays.push(darray.arrays[0]);
          }
        } else if (slice.start === pod.arrays[pod.arrays.length - 1].end) {
          if (
            pod.arrays.length === 1 &&
            (dim as RangeDimension).range[0] === pod.arrays[0].start
          ) {
            // we did retrieve all data
            pod.arrays[0].setHasAllIndices();
            return pod.arrays[0];
          } else {
            pod.setHasAllUpperIndices();
          }
        }
      } else {
        const indicesLastIdx = indices.length;

        // extract and merge overlapping or continous subarrays
        const darray = new ArrayPortionOfData(
          Math.min(slice.start, pod.arrays[startExtractIndex].start)
        );
        indices.push(-1);

        for (
          let idx = slice.start;
          idx < pod.arrays[startExtractIndex].start;
          idx++
        ) {
          indices[indicesLastIdx] = idx;
          const val = fromDataSlice(
            slice.getValue(idx) as DataSliceImpl | null,
            dim!.get(idx) as (Dimension | null),
            data[idx - slice.start],
            indices
          );
          darray.push(val);
        }

        for (let i = startExtractIndex; i < endExtractIndex; i++) {
          const sarray = pod.arrays[i];
          // TODO: optimize me, can do better
          for (let idx = sarray.start; idx < sarray.end; idx++) {
            if (idx >= slice.start && idx < slice.end) {
              indices[indicesLastIdx] = idx;
              const val = addDataSlice(
                sarray.get(idx) as PortionOfDataImpl,
                slice.getValue(idx) as DataSliceImpl | null,
                dim!.get(idx) as Dimension | null,
                data[idx - slice.start],
                indices
              );
              darray.push(val);
            } else {
              darray.push(sarray.get(idx));
            }
          }

          if (i + 1 < endExtractIndex && sarray.end < pod.arrays[i + 1].start) {
            // there is a gap, fill the gap
            for (let idx = sarray.end; idx < pod.arrays[i + 1].start; idx++) {
              indices[indicesLastIdx] = idx;
              const val = fromDataSlice(
                slice.getValue(idx) as DataSliceImpl | null,
                dim!.get(idx) as (Dimension | null),
                data[idx - slice.start],
                indices
              );
              darray.push(val);
            }
          }
        }

        for (
          let idx = pod.arrays[endExtractIndex - 1].end;
          idx < slice.start + data.length;
          idx++
        ) {
          indices[indicesLastIdx] = idx;
          const val = fromDataSlice(
            slice.getValue(idx) as DataSliceImpl | null,
            dim!.get(idx) as (Dimension | null),
            data[idx - slice.start],
            indices
          );
          darray.push(val);
        }

        indices.pop();
        const newArrays = pod.arrays.slice(0, startExtractIndex);
        newArrays.push(darray);
        pod.arrays = newArrays.concat(pod.arrays.slice(endExtractIndex));
      }

      if (
        ((slice.start === slice.end &&
          slice.start === pod.arrays[pod.arrays.length - 1].end) ||
          (data.length > 0 && slice.end > slice.start + data.length)) &&
        !pod.hasAllIndices
      ) {
        // empty slice mean there is no more data
        if (
          pod.arrays.length === 1 &&
          (dim as RangeDimension).range[0] === pod.arrays[0].start
        ) {
          // we did retrieve all data
          pod.arrays[0].setHasAllIndices();
          return pod.arrays[0];
        } else {
          pod.setHasAllUpperIndices();
        }
      }
    } else {
      // ArrayPortionsOfData
      const indicesLastIdx = indices.length;
      indices.push(0);

      for (
        let idx = slice.start, n = slice.start + data.length;
        idx < n;
        idx++
      ) {
        indices[indicesLastIdx] = idx;
        const val = addDataSlice(
          pod.get(idx) as PortionOfDataImpl,
          slice.getValue(idx) as DataSliceImpl | null,
          dim!.get(idx) as (Dimension | null),
          data[idx - slice.start],
          indices
        );
        pod.set(idx, val);
      }

      indices.pop();
    }

    return pod;
  }
}

function isRangeOverlappedOrContinuous(
  start0: number,
  end0: number,
  start1: number,
  end1: number
): boolean {
  return end1 >= start0 && end0 >= start1;
}
