import { readFileSync } from "fs";
import { Grid } from "./grid"

function readInput(filename: string): number[][] {
  return readFileSync(filename, "utf8")
    .toString()
    .split("\n")
    .filter((x) => x.length)
    .map(s => s.split("").map(c => parseInt(c)));
}

export function readHeights(filename: string): Grid {
  return new Grid(readInput(filename));
}
