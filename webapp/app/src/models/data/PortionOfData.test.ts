import {
  PortionOfData,
  DiscontinuousArrayPortionOfData,
  ArrayPortionOfData,
  ContinuousRangeDataSlice,
  DataSlice,
  IndexDataSlice
} from ".";
import { UnitOfData } from "./UnitOfData";
import { RangeDimension } from "./Dimension";
import { IDGenerator } from "../../misc/IDGenerator";

import * as fs from "fs";
import * as parse from "csv-parse/lib/sync";

function deserSlices(val: string): DataSlice {
  const idgen = new IDGenerator();
  const deser = (values: string[]): DataSlice | null => {
    if (values.length === 0) {
      return null;
    }

    if (values[0].includes("..")) {
      const m = values[0].match(/(\d+)?..(\d+)?/);
      if (m === null) {
        throw new Error(`invalid slice: ${values[0]}`);
      }

      return new ContinuousRangeDataSlice(
        `s_${idgen.next()}`,
        [parseInt(m![1] || "0", 10), m![2] ? parseInt(m![2], 10) : Infinity],
        [deser(values.slice(1))]
      );
    } else {
      return new IndexDataSlice(`s_${idgen.next()}`, {
        [values[0]]: deser(values.slice(1))
      });
    }
  };

  return deser(val.split(":"))!;
}

it("toJSON", () => {
  const dim = new RangeDimension(
    "d0",
    0,
    [0, 100],
    [new RangeDimension("d1", 1, [0, 3], [null])]
  );
  const dm = [[1, 2, 3], [6, 7, 8]];
  const pod = PortionOfData.fromSlices(deserSlices("0..2:..3"), dim, dm);
  expect(pod.toJSON()).toEqual(dm);
  expect(
    PortionOfData.fromSlices(deserSlices("3..5:..3"), dim, dm).toJSON()
  ).toEqual([null, null, null, [1, 2, 3], [6, 7, 8]]);
});

it("addData", () => {
  const data = parse(
    fs.readFileSync("../tests/resources/s101_time_row.csv").toString(),
    { skip_empty_lines: true }
  );
  const dimension = new RangeDimension(
    "0",
    0,
    [0, data.length],
    [new RangeDimension("1", 1, [0, data[0].length], [null])]
  );

  let pod = PortionOfData.fromSlices(
    deserSlices("0..10:.."),
    dimension,
    data.slice(0, 10)
  );
  expect(pod.toJSON()).toEqual(data.slice(0, 10));

  pod = pod.addData(deserSlices("20..30:.."), dimension, data.slice(20, 30));
  expect(pod.toJSON()).toEqual(
    data
      .slice(0, 10)
      .concat(new Array(10).fill(null))
      .concat(data.slice(20, 30))
  );

  pod = pod.addData(deserSlices("10..20:.."), dimension, data.slice(10, 20));
  expect(pod.toJSON()).toEqual(data.slice(0, 30));

  pod = pod.addData(deserSlices("30..40:.."), dimension, data.slice(30, 40));
  expect(pod.toJSON()).toEqual(data.slice(0, 40));

  pod = pod.addData(deserSlices("40..:.."), dimension, data.slice(40));
  expect(pod.toJSON()).toEqual(data);
  expect(pod.hasAllIndices).toBeTruthy();
  expect(pod.hasAllUpperIndices).toBeTruthy();

  pod = pod.addData(deserSlices("40..50:..."), dimension, data.slice(40, 42));
  expect(pod.toJSON()).toEqual(data);
  expect(pod.hasAllIndices).toBeTruthy();
  expect(pod.hasAllUpperIndices).toBeTruthy();

  pod = PortionOfData.fromSlices(
    deserSlices("20..30:.."),
    dimension,
    data.slice(20, 30)
  );

  pod = pod.addData(deserSlices("42..42:.."), dimension, []);
  expect(pod.toJSON()).toEqual(
    new Array(20).fill(null).concat(data.slice(20, 30))
  );
  expect(pod.hasAllUpperIndices).toBeFalsy();

  pod = pod.addData(deserSlices("30..42:.."), dimension, data.slice(30, 42));
  expect(pod.toJSON()).toEqual(
    new Array(20).fill(null).concat(data.slice(20, 42))
  );
  expect(pod.hasAllIndices).toBeFalsy();
  expect(pod.hasAllUpperIndices).toBeFalsy();

  pod = pod.addData(deserSlices("42..42:.."), dimension, []);
  expect(pod.toJSON()).toEqual(
    new Array(20).fill(null).concat(data.slice(20, 42))
  );
  expect(pod.hasAllIndices).toBeFalsy();
  expect(pod.hasAllUpperIndices).toBeTruthy();
});
