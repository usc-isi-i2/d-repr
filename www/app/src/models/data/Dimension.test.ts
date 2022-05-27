import { Dimension, RangeDimension, IndexDimension } from "./Dimension";
import { ContinuousRangeDataSlice } from ".";

const dim1 = () =>
  Dimension.deserialize({
    type: "range",
    range: [0, 10, 12, "inf"],
    values: [
      null,
      {
        type: "index",
        values: {
          cities: null,
          cityPopulation: {
            type: "range",
            range: [0, "inf"],
            values: [null]
          }
        }
      },
      null
    ]
  });

it("Test deserialize", () => {
  expect(dim1()).toEqual(
    new RangeDimension(
      "rd_2",
      0,
      [0, 10, 12, Infinity],
      [
        null,
        new IndexDimension("id_1", 1, {
          cities: null,
          cityPopulation: new RangeDimension("rd_0", 2, [0, Infinity], [null])
        }),
        null
      ]
    )
  );
});

it("test getMaxNDims", () => {
  expect(dim1().getMaxNDims()).toEqual(3);
});

it("test validate slices", () => {
  expect(
    dim1().isValidSlice(new ContinuousRangeDataSlice("rd_0", [0, 10], [null]))
  );
});
