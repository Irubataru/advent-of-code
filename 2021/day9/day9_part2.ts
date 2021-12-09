import { readHeights } from "./io";
import { Point, findLowPoints } from "./grid";

function remove<T>(array: T[], value: T) {
  const index = array.indexOf(value);
  if (index > -1) {
    array.splice(index, 1);
  }
}

function has(array: Point[], value: Point): boolean {
  for (var item of array) {
    if (value.equals(item)) {
      return true;
    }
  }
  return false;
}

function addIfUnused(edge: Point[], basin: Point[], x: number, y: number) {
  const testPoint = new Point(x, y, 0);
  if (!has(basin, testPoint) && !has(edge, testPoint)) {
    edge.push(testPoint);
    basin.push(testPoint);
  }
}

var heights = readHeights("input");
var lowPoints = findLowPoints(heights);

var visited: Point[] = []
var sizes: number[] = [];

for (var point of lowPoints) {

  if (has(visited, point)) { continue; }

  var basin: Point[] = [point];
  var edge: Point[] = [point];

  visited.push(point);

  while (edge.length > 0) {
    for (var p of edge) {
      remove(edge, p);

      if (p.x > 0 && heights.get(p.x - 1, p.y) !== 9) {
        addIfUnused(edge, basin, p.x - 1, p.y);
      }

      if (p.x < heights.rows - 1 && heights.get(p.x + 1, p.y) !== 9) {
        addIfUnused(edge, basin, p.x + 1, p.y);
      }

      if (p.y > 0 && heights.get(p.x, p.y - 1) !== 9) {
        addIfUnused(edge, basin, p.x, p.y - 1);
      }

      if (p.y < heights.columns - 1 && heights.get(p.x, p.y + 1) !== 9) {
        addIfUnused(edge, basin, p.x, p.y + 1);
      }
    }
  }

  for (var p of basin) {
    visited.push(p);
  }

  sizes.push(basin.length);
}

sizes.sort((a,b) => b-a);

console.log(sizes[0] * sizes[1] * sizes[2]);
