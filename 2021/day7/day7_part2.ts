import { readInput } from "./io"
import { range } from "./range"

var positions = readInput("input");

var min_pos = positions.reduce((min, next) => Math.min(min, next), 0);
var max_pos = positions.reduce((max, next) => Math.max(max, next), 0);

function fuelConsumptionIncreasing(distance: number) {
  return distance * (distance + 1) / 2;
}

const minimum_consumption = range(min_pos, max_pos).reduce(
  (min, pos) =>
    Math.min(
      min,
      positions.reduce((total, next) => total + fuelConsumptionIncreasing(Math.abs(next - pos)), 0)
    ),
  Number.MAX_SAFE_INTEGER
);

console.log(minimum_consumption);
