import { readFileSync } from "fs";

export function readLines(filename: string): string[] {
  return readFileSync(filename, "utf8")
    .toString()
    .split("\n")
    .filter((x) => x.length);
}

export function readIntegerGrid(
  filename: string,
  delimiter: string
): number[][] {
  return readLines(filename).map((line) =>
    line
      .split(delimiter)
      .filter((x) => x.length > 0)
      .map((str) => parseInt(str))
  );
}
