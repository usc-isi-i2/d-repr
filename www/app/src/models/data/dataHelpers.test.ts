import { RangeUtil } from "./dataHelpers";

it("RangeUtil.changeRange", () => {
  const ru = new RangeUtil([5, 7, 19], ["a", "b"]);

  expect(() => ru.changeRange(3, 10)).toThrow("Invalid start");
  expect(() => ru.changeRange(20, 10)).toThrow("Invalid start");
  expect(ru.changeRange(6, 10)).toEqual(new RangeUtil([6, 7, 16], ["a", "b"]));
  expect(ru.changeRange(6, 13)).toEqual(new RangeUtil([6, 7, 19], ["a", "b"]));
  expect(ru.changeRange(6, 15)).toEqual(new RangeUtil([6, 7, 19], ["a", "b"]));
});

it("RangeUtil.validate", () => {
  expect(new RangeUtil([5, 7, 6, 19], ["a", "b", "c"]).validate()).toBeFalsy();
});

it("RangeUtil.optimize", () => {
  expect(new RangeUtil([5, 7, 9, 19], ["a", "b", "b"]).optimize()).toEqual(
    new RangeUtil([5, 7, 19], ["a", "b"])
  );
  expect(
    new RangeUtil([5, 6, 7, 9, 13, 19], ["a", "a", "b", "b", "c"]).optimize()
  ).toEqual(new RangeUtil([5, 7, 13, 19], ["a", "b", "c"]));
});
