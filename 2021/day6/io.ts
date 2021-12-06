import * as fs from "fs";
import { LanternFish } from "./fish";

export function readAges(filename: string): number[] {
  return fs
    .readFileSync(filename, "utf8")
    .toString()
    .split(",")
    .filter((x) => x.length)
    .map((s) => parseInt(s));
}

export function readFish(filename: string): LanternFish[] {
  return readAges(filename).map((i) => new LanternFish(i));
}
