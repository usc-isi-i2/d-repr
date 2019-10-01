import {
  DataSlice,
  Index,
  IndexDimension,
  PortionOfData,
  UnitOfData,
  Dimension
} from ".";

type II<X> = IterableIterator<X>;
type PUData = PortionOfData | UnitOfData;

export class IndexDataSlice extends DataSlice {
  public index2slice: { [idx in Index]: DataSlice | null };

  constructor(id: string, index2slice: { [idx in Index]: DataSlice | null }) {
    super(id);
    this.index2slice = index2slice;
  }

  public isSelected(idx: Index) {
    return idx in this.index2slice;
  }

  public setValue(idx: Index, val: DataSlice | null) {
    this.index2slice[idx] = val;
  }

  public getValue(idx: Index) {
    return this.index2slice[idx];
  }

  public isSlicingAllIndices(dim: Dimension): boolean {
    if (dim instanceof IndexDimension) {
      return (
        new Set(Object.keys(this.index2slice)) ===
        new Set(Object.keys(dim.index2val))
      );
    }

    throw new Error("Invalid type of dim, expected to be RangeDimension");
  }

  public isSelectOne(): boolean {
    return Object.keys(this.index2slice).length === 1;
  }

  public toIndex(): Index {
    if (this.isSelectOne()) {
      return Object.keys(this.index2slice)[0];
    }
    throw new Error(
      "Cannot convert the current slice into an index because it selects more than one element"
    );
  }

  public *iter(data: PortionOfData): II<PortionOfData | UnitOfData> {
    for (const k of Object.keys(this.index2slice)) {
      yield data[k];
    }
  }

  public *iterKeys(data: PortionOfData): IterableIterator<Index> {
    for (const k of Object.keys(this.index2slice)) {
      yield k;
    }
  }

  public *iterEntries(data: PortionOfData): II<[Index, PUData]> {
    for (const k of Object.keys(this.index2slice)) {
      yield [k, data[k]];
    }
  }

  public *iterDFS(depth: number = 0): II<[DataSlice, number]> {
    yield [this, depth];

    for (const idx in this.index2slice) {
      const val = this.index2slice[idx];
      if (val === null) {
        continue;
      }

      for (const [s, d] of val.iterDFS(depth + 1)) {
        yield [s, d];
      }
    }
  }

  public *iterDim(dim: Dimension): II<[DataSlice, Dimension]> {
    yield [this, dim];

    for (const idx in this.index2slice) {
      const val = this.index2slice[idx];
      if (val === null) {
        continue;
      }

      for (const [s, d] of val.iterDim(dim.get(idx)!)) {
        yield [s, d];
      }
    }
  }

  public serialize(): object {
    const obj: any = {
      type: "index",
      index2slice: []
    };
    for (const k in this.index2slice) {
      // TODO: fix me! hard code to fix the issue that javascript always convert number to string
      // in object keys
      obj.index2slice.push([
        isNaN(k as any) ? k : parseInt(k, 10),
        this.index2slice[k] !== null ? this.index2slice[k]!.serialize() : null
      ]);
    }
    return obj;
  }

  public clone(): DataSlice {
    const obj = {};
    for (const k in this.index2slice) {
      obj[k] =
        this.index2slice[k] === null ? null : this.index2slice[k]!.clone();
    }

    return new IndexDataSlice(this.id, obj);
  }
}
