import { readFileSync } from "fs";

export function readInput(filename: string): string[][] {
  return readFileSync(filename, "utf8")
    .toString()
    .split("\n")
    .filter((x) => x.length)
    .map(s => s.split(""));
}
