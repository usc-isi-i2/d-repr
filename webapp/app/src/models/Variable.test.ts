import { Location, SLICE_UNSET, RangeSlice, IndexSlice } from "./Variable";
import { Dimension } from "./data";

describe("Location", () => {
  const complexDim = Dimension.deserialize({
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
                  phone: null,
                  friends: {
                    type: "range",
                    range: [0, 20],
                    values: [null]
                  }
                }
              }
            ]
          },
          company: null
        }
      }
    ]
  });
  const dim2d = Dimension.deserialize({
    type: "range",
    range: [0, 100],
    values: [
      {
        type: "range",
        range: [0, 50],
        values: [null]
      }
    ]
  });

  it("Location.fromString", () => {
    expect(
      Location.fromString("resource-0", "0..3:artists:0..25:name")
    ).toEqual(
      new Location("resource-0", [
        new RangeSlice(0, 3, 1),
        new IndexSlice("artists"),
        new RangeSlice(0, 25, 1),
        new IndexSlice("name")
      ])
    );
    expect(Location.fromString("resource-0", "0..1:")).toEqual(
      new Location("resource-0", [new RangeSlice(0, 1, 1), new IndexSlice("")])
    );
  });

  it("Location.validate", () => {
    for (const loc of [
      "0..3:artists:0..25:name",
      "0..:artists:0..25:name",
      "0..:artists:0..:name",
      "0..:artists:0..:friends:.."
    ]) {
      expect(
        Location.fromString("resource-0", loc).validate(complexDim)
      ).toBeTruthy();
    }

    for (const loc of [
      "0..3",
      "0..",
      "0..:artists:0..101:name",
      "0..:artists:0..:friends"
    ]) {
      expect(
        Location.fromString("resource-0", loc).validate(complexDim)
      ).toBeFalsy();
    }

    for (const loc of ["0..1:", "0", "0..1"]) {
      expect(
        Location.fromString("resource-0", loc).validate(dim2d)
      ).toBeFalsy();
    }
  });
});
