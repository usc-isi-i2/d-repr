import {
  DataSlice,
  ContinuousRangeDataSlice,
  Dimension,
  slices2DataSlices
} from ".";
import { IndexDataSlice } from "./IndexDataSlice";
import * as _ from "lodash";
import { Location } from "../Variable";
import { IDGenerator } from "../../misc/IDGenerator";

describe("test create data slice", () => {
  const ds = DataSlice.deserialize({
    type: "range",
    range: [0, 10, 100],
    values: [
      null,
      {
        type: "index",
        index2slice: {
          name: null,
          phone: {
            type: "range",
            range: [0, 10],
            values: [null]
          }
        }
      }
    ]
  });

  it("create data slice", () => {
    // array resource
    let ds0;
    let ds1;
    ds0 = new ContinuousRangeDataSlice("d0", [0, 100], [null]);
    expect([ds0.range, ds0.values]).toEqual([[0, 100], [null]]);
    expect(ds0.clone()).toEqual(ds0);
    expect(ds0.clone() === ds0).not.toBeTruthy();

    // csv resource
    ds1 = new ContinuousRangeDataSlice("d1", [0, 5], [null]);
    ds0 = new ContinuousRangeDataSlice("d0", [0, 100], [ds1]);
    expect([ds0.range, ds0.values]).toEqual([[0, 100], [ds1]]);
    expect([ds1.range, ds1.values]).toEqual([[0, 5], [null]]);
    expect(ds0.clone()).toEqual(ds0);
    expect(ds0.clone() === ds0).not.toBeTruthy();
  });

  it("deserialize data slice", () => {
    expect(ds).toEqual(
      new ContinuousRangeDataSlice(
        "rds_0",
        [0, 10, 100],
        [
          null,
          new IndexDataSlice("ids_1", {
            name: null,
            phone: new ContinuousRangeDataSlice("rds_2", [0, 10], [null])
          })
        ]
      )
    );
  });

  it("serialize data slice", () => {
    expect(ds.serialize()).toEqual({
      type: "range",
      range: [0, 10, 100],
      values: [
        null,
        {
          type: "index",
          index2slice: {
            name: null,
            phone: {
              type: "range",
              range: [0, 10],
              values: [null]
            }
          }
        }
      ]
    });
  });
});

describe("test iterate the data", () => {
  it("iterDFS", () => {
    const ds0 = DataSlice.deserialize({
      type: "range",
      range: [0, 10, 100],
      values: [
        null,
        {
          type: "index",
          index2slice: {
            name: null,
            phone: {
              type: "range",
              range: [0, 10],
              values: [null]
            }
          }
        }
      ]
    });

    const result = _.map(Array.from(ds0.iterDFS()), r => [r[0].id, r[1]]);
    expect(result).toEqual([
      [ds0.id, 0],
      [ds0.getValue(10)!.id, 1],
      [ds0.getValue(10)!.getValue("phone")!.id, 2]
    ]);
  });
});

describe("create data slice", () => {
  const dim = Dimension.deserialize({
    type: "range",
    range: [0, 100],
    values: [
      {
        type: "index",
        values: {
          artists: {
            type: "range",
            range: [0, 100],
            values: [
              {
                type: "index",
                values: {
                  name: null,
                  phone: null
                }
              }
            ]
          },
          company: null
        }
      }
    ]
  });

  it("predict from data slice", () => {
    const loc = Location.fromSerializedSlices("resource-0", [
      "5..8",
      "artists",
      "0..25",
      "name"
    ]);

    const ds = slices2DataSlices(0, loc.slices, new IDGenerator())!;
    expect(ds.serialize()).toEqual({
      type: "range",
      range: [5, 8],
      values: [
        {
          type: "index",
          index2slice: {
            artists: {
              type: "range",
              range: [0, 25],
              values: [
                {
                  type: "index",
                  index2slice: {
                    name: null
                  }
                }
              ]
            }
          }
        }
      ]
    });

    expect(
      DataSlice.fromSlices("resource-0", dim, loc.slices).serialize()
    ).toEqual({
      type: "range",
      range: [5, 15],
      values: [
        {
          type: "index",
          index2slice: {
            company: null,
            artists: {
              type: "range",
              range: [0, 10],
              values: [
                {
                  type: "index",
                  index2slice: {
                    name: null,
                    phone: null
                  }
                }
              ]
            }
          }
        }
      ]
    });
  });

  it("predict from dimension", () => {
    expect(DataSlice.fromDimension("resource-0", dim).serialize()).toEqual({
      type: "range",
      range: [0, 10],
      values: [
        {
          type: "index",
          index2slice: {
            company: null,
            artists: {
              type: "range",
              range: [0, 10],
              values: [
                {
                  type: "index",
                  index2slice: {
                    name: null,
                    phone: null
                  }
                }
              ]
            }
          }
        }
      ]
    });
  });
});
