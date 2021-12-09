export class Grid {
  heights: number[][];
  rows: number;
  columns: number;

  constructor(values: number[][]) {
    this.rows = values.length;
    this.columns = values[0].length;
    this.heights = values;
  }

  get(i: number, j: number): number {
    return this.heights[i][j];
  }

  toString(): string {
    return this.heights.toString();
  }
}

export class Point {
  x: number;
  y: number;
  h: number;

  constructor(x: number, y: number, h: number) {
    this.x = x;
    this.y = y;
    this.h = h;
  }

  toString() {
    return `(${this.x},${this.y}:${this.h})`;
  }

  equals(other: Point): boolean {
    return this.x === other.x && this.y === other.y;
  }
}

export function findLowPoints(heights: Grid): Point[] {
  var result: Point[] = [];

  for (var i = 0; i < heights.rows; i++) {
    for (var j = 0; j < heights.columns; j++) {
      if (i > 0 && heights.get(i - 1, j) <= heights.get(i, j)) {
        continue;
      }

      if (i < heights.rows - 1 && heights.get(i + 1, j) <= heights.get(i, j)) {
        continue;
      }

      if (j > 0 && heights.get(i, j - 1) <= heights.get(i, j)) {
        continue;
      }

      if (
        j < heights.columns - 1 &&
        heights.get(i, j + 1) <= heights.get(i, j)
      ) {
        continue;
      }

      result.push(new Point(i, j, heights.get(i, j)));
    }
  }

  return result;
}
