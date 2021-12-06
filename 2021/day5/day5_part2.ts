import { Grid } from "./grid";
import { readLines } from "./io";
import { LineType, placeLines, countOverlapping } from "./vent_lines";

const lines = readLines("input");
const grid = placeLines(
  lines,
  new Grid<number>(1000, 0),
  LineType.VerticalLine | LineType.HorizontalLine | LineType.DiagonalLine
);

console.log(`Overlapping points: ${countOverlapping(grid)}`);
