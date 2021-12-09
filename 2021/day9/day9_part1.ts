import { readHeights } from "./io";
import { findLowPoints } from "./grid";

console.log(
  findLowPoints(readHeights("input")).reduce((tot, val) => tot + val.h + 1, 0)
);
