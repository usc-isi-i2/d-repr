import { IDGenerator } from "../../misc/IDGenerator";
import { Slice, RangeSlice } from "../Variable";
import {
  PortionOfData,
  UnitOfData,
  Dimension,
  ContinuousRangeDataSlice,
  IndexDataSlice,
  Index,
  RangeUtil,
  RangeDimension,
  DimensionImpl
} from ".";
import * as _ from "lodash";

type PUData = PortionOfData | UnitOfData;
type II<X> = IterableIterator<X>;

export abstract class DataSlice {
  public static deserialize(obj: any): DataSlice {
    return deserializeDataSlice(obj, new IDGenerator());
  }

  public static fromSlices(
    resourceId: string,
    dim: Dimension,
    slices: Slice[]
  ): DataSlice {
    const dslice = slices2DataSlices(0, slices, new IDGenerator())!;
    restrainDataSliceFixedCostModel(dslice, dim, 1000, 10, 20);
    return dslice;
  }

  public static fromDimension(resourceId: string, dim: Dimension): DataSlice {
    return dimension2DataSlicesFixCostModel(
      dim instanceof RangeDimension ? dim.range[0] : Infinity,
      dim as DimensionImpl,
      new IDGenerator(),
      2000,
      10,
      30
    );
  }

  public readonly id: string;

  constructor(id: string) {
    this.id = id;
  }

  // test if data slice is overlapped with slices, the slices should be checked to make sure it is valid before calling this function
  public isOverlappedWithSlices(slices: Slice[]): boolean {
    for (const [sptr, depth] of this.iterDFS()) {
      const slice = slices[depth];
      if (sptr instanceof ContinuousRangeDataSlice) {
        if (slice instanceof RangeSlice) {
          if (!slice.isRangeOverlapped(sptr.start, sptr.end)) {
            return false;
          }
        } else {
          if (!(slice.index >= sptr.start && slice.index < sptr.end)) {
            return false;
          }
        }
      } else {
        if (slice instanceof RangeSlice) {
          let isOverlapped = false;
          for (const idx in (sptr as IndexDataSlice).index2slice) {
            if (typeof idx === "number" && slice.isSelected(idx)) {
              isOverlapped = true;
              break;
            }
          }

          if (!isOverlapped) {
            return false;
          }
        } else {
          if (!(slice.index in (sptr as IndexDataSlice).index2slice)) {
            return false;
          }
        }
      }
    }

    return true;
  }

  // if an index is selected in this data slice
  public abstract isSelected(idx: Index): boolean;
  // set nested data slice
  public abstract setValue(idx: Index, val: DataSlice | null): void;
  // get nested data slice
  public abstract getValue(idx: Index): DataSlice | null;
  // test if the slice select all indices in the given dimension (non-recursive)
  public abstract isSlicingAllIndices(dim: Dimension): boolean;
  // test if this slice is select only one element
  public abstract isSelectOne(): boolean;
  // convert the slice into an index if it only select one element in the current dimension
  public abstract toIndex(): Index;
  // iterate through all values of data in this dimension
  public abstract iter(data: PortionOfData): II<PUData>;
  // iterate through all values in this dimension (we need data because the slice can be infinity)
  public abstract iterKeys(data: PortionOfData): II<Index>;
  // iterate through all index and values in this dimension
  public abstract iterEntries(data: PortionOfData): II<[Index, PUData]>;
  // iterate through nested data slice (DFS)
  public abstract iterDFS(depth?: number): II<[DataSlice, number]>;
  // iterate through nested dimension (DFS)
  public abstract iterDim(dim: Dimension): II<[DataSlice, Dimension]>;
  public abstract serialize(): object;
  public abstract clone(): DataSlice;
}

export type DataSliceImpl = ContinuousRangeDataSlice | IndexDataSlice;

function deserializeDataSlice(obj: any, idgen: IDGenerator): DataSlice {
  if (obj.type === "range") {
    return new ContinuousRangeDataSlice(
      `rds_${idgen.next()}`,
      obj.range,
      _.map(obj.values, v =>
        v === null ? null : deserializeDataSlice(v, idgen)
      )
    );
  } else {
    const id = `ids_${idgen.next()}`;
    const index2slice = {};
    for (const [idx, val] of obj.index2slice) {
      if (obj.index2slice[idx] !== null) {
        index2slice[idx] = deserializeDataSlice(val, idgen);
      } else {
        index2slice[idx] = null;
      }
    }
    return new IndexDataSlice(id, index2slice);
  }
}

export function slices2DataSlices(
  sliceIndex: number,
  slices: Slice[],
  idgen: IDGenerator
): DataSlice | null {
  if (sliceIndex >= slices.length) {
    return null;
  }

  const slice = slices[sliceIndex];
  if (slice instanceof RangeSlice) {
    const id = `rds_${idgen.next()}`;
    const range = [slice.start, slice.end];
    const values = [slices2DataSlices(sliceIndex + 1, slices, idgen)];

    return new ContinuousRangeDataSlice(id, range, values);
  } else {
    return new IndexDataSlice(`ids_${idgen.next()}`, {
      [slice.index]: slices2DataSlices(sliceIndex + 1, slices, idgen)
    });
  }
}

function dimension2DataSlicesFixCostModel(
  start: number, // to only set
  dim: DimensionImpl,
  idgen: IDGenerator,
  maxNElements: number,
  maxElementPerRangeDim: number,
  maxElementPerIndexDim: number
): DataSlice {
  if (dim instanceof RangeDimension) {
    const id = `rds_${idgen.next()}`;
    let ru = new RangeUtil(dim.range, dim.values);
    ru = ru.changeRange(start, maxElementPerRangeDim);
    const range = ru.range;
    const values = _.map(ru.values, v => {
      if (v === null) {
        return null;
      }

      return dimension2DataSlicesFixCostModel(
        v instanceof RangeDimension ? v.range[0] : Infinity,
        v,
        idgen,
        maxNElements,
        // TODO: fix me! hard code here to select > 10 columns
        20,
        maxElementPerIndexDim
      );
    });

    return new ContinuousRangeDataSlice(id, range, values);
  } else {
    const id = `ids_${idgen.next()}`;
    const index2slice = {};
    let val;

    for (const idx in dim.index2val) {
      if (_.size(index2slice) > maxElementPerIndexDim) {
        break;
      }

      val = dim.index2val[idx];
      if (val === null) {
        index2slice[idx] = null;
      } else {
        index2slice[idx] = dimension2DataSlicesFixCostModel(
          val instanceof RangeDimension ? val.range[0] : Infinity,
          val,
          idgen,
          maxNElements,
          maxElementPerRangeDim,
          maxElementPerIndexDim
        );
      }
    }

    return new IndexDataSlice(id, index2slice);
  }
}

function restrainDataSliceFixedCostModel(
  dslice: DataSlice,
  dim: Dimension,
  maxNElements: number,
  maxElementPerRangeDim: number,
  maxElementPerIndexDim: number
) {
  // apply the following heuristic: apply in reverse direction (from bottom to top)
  // if the size is too big, we shrink it down, if the size is too small, be make it bigger
  // until the total selected is not exceeded certain size
  const idgen = new IDGenerator();
  // mapping between depth to data slices
  const index = {};
  let maxDepth = 0;
  for (const [sptr, dptr] of dslice.iterDim(dim)) {
    if (!(dptr.depth in index)) {
      index[dptr.depth] = [];
    }

    index[dptr.depth].push([sptr, dptr]);
    if (dptr.depth > maxDepth) {
      maxDepth = dptr.depth;
    }
  }

  // start work from bottom to top
  for (let d = maxDepth; d >= 0; d--) {
    // examine each data slice
    for (const [sptr, dptr] of index[d]) {
      if (sptr instanceof ContinuousRangeDataSlice) {
        if (sptr.end - sptr.start > maxElementPerRangeDim) {
          // reduce the size
          const ru = new RangeUtil(sptr.range, sptr.values).changeRange(
            sptr.start,
            maxElementPerRangeDim
          );
          sptr.range = ru.range;
          sptr.values = ru.values;
        } else if (sptr.end - sptr.start < maxElementPerRangeDim) {
          // increase the size
          const child = dimension2DataSlicesFixCostModel(
            sptr.end,
            dptr,
            idgen,
            maxNElements,
            maxElementPerRangeDim,
            maxElementPerRangeDim
          ) as ContinuousRangeDataSlice;

          const ru = new RangeUtil(
            sptr.range.concat(child.range.slice(1)),
            sptr.values.concat(child.values)
          )
            .changeRange(sptr.start, maxElementPerRangeDim)
            .optimize();
          sptr.range = ru.range;
          sptr.values = ru.values;
        }
      } else {
        const nIndex = _.size(sptr.index2slice);
        if (nIndex > maxElementPerIndexDim) {
          // reduce the size
          const deletedIds = Object.keys(sptr.index2slice).slice(
            0,
            maxElementPerIndexDim - nIndex
          );
          for (const idx of deletedIds) {
            delete sptr.index2slice[idx];
          }
        } else if (nIndex < maxElementPerIndexDim) {
          // increase the size
          let nIncrease = maxElementPerIndexDim - nIndex;
          let val;
          for (const idx in dptr.index2val) {
            if (!(idx in sptr.index2slice)) {
              val = dptr.index2val[idx];
              if (val === null) {
                sptr.index2slice[idx] = null;
              } else {
                sptr.index2slice[idx] = dimension2DataSlicesFixCostModel(
                  Infinity,
                  val,
                  idgen,
                  maxNElements,
                  maxElementPerRangeDim,
                  maxElementPerIndexDim
                );
              }
              if (--nIncrease === 0) {
                break;
              }
            }
          }
        }
      }
    }
  }
}
