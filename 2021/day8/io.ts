import { readFileSync } from "fs";
import { DisplayValue } from "./display";

export class Input {
  patterns: DisplayValue[] = [];
  displays: DisplayValue[] = [];
  raw: string;

  constructor(patterns: DisplayValue[], displays: DisplayValue[], raw: string) {
    this.patterns = patterns;
    this.displays = displays;
    this.raw = raw;
  }

  public toString(): string {
    return this.raw;
  }
}

function mapChar(char: string): number {
  return char.charCodeAt(0) - "a".charCodeAt(0);
}

function readLines(filename: string): string[] {
  return readFileSync(filename, "utf8")
    .toString()
    .split("\n")
    .filter((x) => x.length);
}

function parseInput(line: string): Input {
  let [lhs, rhs] = line.split("|").map((s) => s.trim());
  var patterns = lhs
    .split(" ")
    .filter((s) => s.length > 0)
    .map(
      (s) =>
        new DisplayValue(
          s
            .split("")
            .map((c) => mapChar(c))
        )
    );
  var displays = rhs
    .split(" ")
    .filter((s) => s.length > 0)
    .map(
      (s) =>
        new DisplayValue(
          s
            .split("")
            .map((c) => mapChar(c))
        )
    );

    return new Input(patterns, displays, line);
}

export function readInput(filename: string): Input[] {
  return readLines(filename).map((line) => parseInput(line));
}
