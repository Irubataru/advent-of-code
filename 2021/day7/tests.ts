import { readFileSync } from "fs";

function readInput(filename: string): number[] {
  return readFileSync(filename, "utf8")
    .toString()
    .split(",")
    .filter((x) => x.length)
    .map((s) => parseInt(s));
}

const positions = readInput("input_dem");

for (var i = 0; i < 20; i++) {

  const rightOf = positions.reduce((total, next) => total + ((next > i) ? 1 : 0), 0);
  const equal = positions.reduce((total, next) => total + ((next === i) ? 1 : 0), 0);
  const leftOf = positions.length - rightOf - equal;

  const consumption = positions.reduce((total, next) => total + Math.abs(next - i), 0)

  console.log(`${i} => ${consumption} (${leftOf}|${equal}|${rightOf})`);

}
console.log(positions.reduce((next, pos) => next + pos, 0) / positions.length);
