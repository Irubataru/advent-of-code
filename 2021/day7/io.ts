import { readFileSync } from "fs";

export function readInput(filename: string): number[] {
  return readFileSync(filename, "utf8")
    .toString()
    .split(",")
    .filter((x) => x.length)
    .map((s) => parseInt(s));
}
