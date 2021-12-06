import { Grid } from "./grid";
import { Line } from "./geometry";

export enum LineType {
  HorizontalLine = 1 << 0,
  VerticalLine = 1 << 1,
  DiagonalLine = 2 << 1,
}

function increment(grid: Grid<number>, i: number, j: number): void {
  grid.setItem(i, j, grid.getItem(i, j) + 1);
}

function isHorizontalLine(line: Line, type: LineType): boolean {
  return (
    (type & LineType.HorizontalLine) == LineType.HorizontalLine &&
    line.vector.y == 0
  );
}

function isVerticalLine(line: Line, type: LineType): boolean {
  return (
    (type & LineType.VerticalLine) == LineType.VerticalLine &&
    line.vector.x == 0
  );
}

function isDiagonalLine(line: Line, type: LineType): boolean {
  return (
    (type & LineType.DiagonalLine) == LineType.DiagonalLine &&
    Math.abs(line.vector.x) == Math.abs(line.vector.y)
  );
}

export function placeLines(
  lines: Line[],
  grid: Grid<number>,
  type: LineType
): Grid<number> {
  for (var line of lines) {
    if (isHorizontalLine(line, type)) {
      const steps = Math.abs(line.vector.x);
      const step = line.vector.x / steps;

      for (var i = 0; i <= steps; i++) {
        increment(grid, line.point.x + step * i, line.point.y);
      }
    } else if (isVerticalLine(line, type)) {
      const steps = Math.abs(line.vector.y);
      const step = line.vector.y / steps;

      for (var i = 0; i <= steps; i++) {
        increment(grid, line.point.x, line.point.y + step * i);
      }
    } else if (isDiagonalLine(line, type)) {
      const steps = Math.abs(line.vector.y);
      const stepX = line.vector.x / steps;
      const stepY = line.vector.y / steps;

      for (var i = 0; i <= steps; i++) {
        increment(grid, line.point.x + stepX * i, line.point.y + stepY * i);
      }
    }
  }

  return grid;
}

export function countOverlapping(grid: Grid<number>): number {
  var overlapping = 0;
  for (var i = 0; i < grid.getSize(); i++) {
    for (var j = 0; j < grid.getSize(); j++) {
      if (grid.getItem(i, j) > 1) {
        overlapping++;
      }
    }
  }

  return overlapping;
}
