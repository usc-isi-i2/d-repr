import {
  DataSlice,
  Index,
  DataSliceIndex,
  ContinuousRangeDataSlice,
  PortionOfData,
  NDimData,
  RangeDimension,
  DimensionIndex
} from "src/models/data";
import { Slice, SLICE_UNSET, IndexSlice, RangeSlice } from "src/models";
import * as _ from "lodash";

interface UnboundDim {
  dsliceId: string;
  sliceIdx: number;
  dimId: string;
}

export class DataView2D {
  public static fromData(resourceId: string, data: NDimData) {
    const dslice = DataSlice.fromDimension(resourceId, data.dimension);

    const slices = [];
    const unboundDims: UnboundDim[] = [];
    // by support only 2 unbound dimensions, we ensure that slices is a sequence
    for (const [sptr, dptr] of dslice.iterDim(data.dimension)) {
      if (sptr instanceof ContinuousRangeDataSlice) {
        unboundDims.push({
          dsliceId: sptr.id,
          sliceIdx: slices.length,
          dimId: dptr.id
        });
        slices.push(
          dptr instanceof RangeDimension
            ? RangeSlice.default()
            : IndexSlice.default()
        );
      } else {
        slices.push(new IndexSlice(sptr.toIndex()));
      }
    }

    if (unboundDims.length !== 2) {
      throw Error("Can only create slices for 2D unbound dimensions");
    }

    return new DataView2D(
      data,
      dslice,
      unboundDims as [UnboundDim, UnboundDim],
      slices
    );
  }

  public readonly data: NDimData;
  public readonly unboundDims: [UnboundDim, UnboundDim];
  public readonly selectedSlices: Slice[];
  public readonly dslice: DataSlice;
  private dataDimIndex: DimensionIndex;

  constructor(
    data: NDimData,
    dslice: DataSlice,
    unboundDims: [UnboundDim, UnboundDim],
    selectedSlices: Slice[]
  ) {
    this.data = data;
    this.dslice = dslice;
    this.unboundDims = unboundDims;
    this.selectedSlices = selectedSlices;
    this.dataDimIndex = new DimensionIndex(data.dimension);
  }

  // number of element in first dimension
  public getNRows(): number | undefined {
    const stopDSlice = this.unboundDims[0].dsliceId;
    let dptr = this.data.pod;

    for (const [sptr, dimptr] of this.dslice.iterDim(this.data.dimension)) {
      if (sptr.id === stopDSlice) {
        if (dimptr.size() === undefined) {
          // we cannot determine the size of the unbound dimension because it depends on previous dimension
          // we have to rely on the fetched data
          if (dptr.hasAllUpperIndices) {
            return dptr.portionSize();
          }

          // we actually don't know the number of page, but Infinity doesn't work with AntDTable, so we are going
          // to return undefined
          return undefined;
        } else {
          return dimptr.size() as number;
        }
      } else {
        dptr = dptr.get(sptr.toIndex()) as PortionOfData;
      }
    }

    throw new Error(
      "Congrat! You found a bug in which we are trying to determine the number of rows. Please report it"
    );
  }

  // change the displaying elements in first dimension
  public getRowsView(startRowIdx: number, endRowIdx: number): DataSlice {
    // TODO: fix me! to change a data slice, it actually require knowledge about the data
    // because every row can have different types
    const dslice = this.dslice.clone();
    const s = DataSliceIndex.getWithoutIndex(
      dslice,
      this.unboundDims[0].dsliceId
    );

    if (s instanceof ContinuousRangeDataSlice) {
      // stupid code
      if (s.range.length !== 2) {
        throw new Error("NotImplementedYet");
      }
      s.range = [startRowIdx, endRowIdx];
    } else {
      throw new Error("Doesn't know how to navigate pages in index page yet");
    }

    return dslice;
  }

  // iterate through values in unbound dimension ${idx}
  public iterUnboundDimensionValues(idx: number) {
    return DataSliceIndex.getWithoutIndex(
      this.dslice,
      this.unboundDims[idx].dsliceId
    ).iterKeys(this.data.pod);
  }

  // clear all selection in this data view
  public removeSelectedSlices(): DataView2D {
    const selectedSlices = _.map(this.selectedSlices, s => s.clone());
    clearSelectedSlicesMut(selectedSlices, this.unboundDims);
    return new DataView2D(
      this.data,
      this.dslice,
      this.unboundDims,
      selectedSlices
    );
  }

  // test if a cell in an unbounded region is selected
  public isSelected(ridx: Index, cidx: Index): boolean {
    if (this.isNoSelection()) {
      return false;
    }

    const rs = this.selectedSlices[this.unboundDims[0].sliceIdx];
    if (rs instanceof RangeSlice) {
      if (!rs.isSelected(ridx as number)) {
        return false;
      }
    } else {
      if (rs.index !== ridx) {
        return false;
      }
    }

    const cs = this.selectedSlices[this.unboundDims[1].sliceIdx];
    if (cs instanceof RangeSlice) {
      if (!cs.isSelected(cidx as number)) {
        return false;
      }
    } else {
      if (cs.index !== cidx) {
        return false;
      }
    }

    return true;
  }

  // user click on a cell in an unbound region return a new selected slices
  public click(indice: Index[]): Slice[] {
    const selectedSlices = _.map(this.selectedSlices, s => s.clone());
    const ridx = indice[this.unboundDims[0].sliceIdx];
    const cidx = indice[this.unboundDims[1].sliceIdx];

    let rs = selectedSlices[this.unboundDims[0].sliceIdx];
    let cs = selectedSlices[this.unboundDims[1].sliceIdx];

    if (this.isNoSelection()) {
      // select first cell
      if (ridx === Infinity || cidx === Infinity) {
        // cannot select infinity
        return selectedSlices;
      }

      if (rs instanceof RangeSlice) {
        rs.start = ridx as number;
        rs.end = rs.start + 1;
      } else {
        rs.index = ridx;
      }

      if (cs instanceof RangeSlice) {
        cs.start = cidx as number;
        cs.end = cs.start + 1;
      } else {
        cs.index = cidx;
      }

      return selectedSlices;
    }

    if (this.isOneCellSelected()) {
      // select a range cell
      let clickOnSameCell = true;

      if (rs instanceof RangeSlice) {
        if ((ridx as number) < rs.start) {
          rs.end = rs.start + 1;
          rs.start = ridx as number;
        } else {
          if (ridx === Infinity) {
            rs.end = Infinity;
          } else {
            rs.end = (ridx as number) + 1;
          }
        }

        if (!rs.isSelectOne()) {
          clickOnSameCell = false;
        }
      } else {
        if (ridx !== rs.index) {
          clickOnSameCell = false;
        }

        // convert IndexSlice into RangeSlice. TODO: check if we can actually do that based on the dimension
        if (
          this.dataDimIndex.get(this.unboundDims[0].dimId) instanceof
          RangeDimension
        ) {
          rs = new RangeSlice(rs.index as number, (ridx as number) + 1, 1);
          selectedSlices[this.unboundDims[0].sliceIdx] = rs;
        } else {
          rs.index = ridx;
        }
      }

      if (cs instanceof RangeSlice) {
        if ((cidx as number) < cs.start) {
          cs.end = cs.start + 1;
          cs.start = cidx as number;
        } else {
          if (cidx === Infinity) {
            cs.end = Infinity;
          } else {
            cs.end = (cidx as number) + 1;
          }
        }

        if (!cs.isSelectOne()) {
          clickOnSameCell = false;
        }
      } else {
        if (cidx !== cs.index) {
          clickOnSameCell = false;
        }

        if (
          this.dataDimIndex.get(this.unboundDims[1].dimId) instanceof
          RangeDimension
        ) {
          cs = new RangeSlice(cs.index as number, (cidx as number) + 1, 1);
          selectedSlices[this.unboundDims[1].sliceIdx] = cs;
        } else {
          cs.index = cidx;
        }
      }

      if (clickOnSameCell) {
        clearSelectedSlicesMut(selectedSlices, this.unboundDims);
      }

      return selectedSlices;
    }

    clearSelectedSlicesMut(selectedSlices, this.unboundDims);
    return selectedSlices;
  }

  // test if a user doesn't select anything in an unbounded region
  private isNoSelection(): boolean {
    const slice = this.selectedSlices[this.unboundDims[0].sliceIdx];
    if (slice instanceof IndexSlice) {
      return slice.index === SLICE_UNSET;
    }
    return slice.start === SLICE_UNSET;
  }

  // test if a user only click on one isOneCellSelected
  private isOneCellSelected(): boolean {
    if (this.isNoSelection()) {
      return false;
    }

    return (
      this.selectedSlices[this.unboundDims[0].sliceIdx].isSelectOne() &&
      this.selectedSlices[this.unboundDims[1].sliceIdx].isSelectOne()
    );
  }
}

function clearSelectedSlicesMut(
  slices: Slice[],
  unboundDims: [UnboundDim, UnboundDim]
): void {
  for (const unboundDim of unboundDims) {
    const s = slices[unboundDim.sliceIdx];
    if (s instanceof RangeSlice) {
      s.start = SLICE_UNSET;
      s.end = SLICE_UNSET;
    } else {
      s.index = SLICE_UNSET;
    }
  }
}
