export class Grid<T> {
  items: T[][];
  rows: number;
  columns: number;

  constructor(
    rows: number,
    columns: number,
    valueConstructor: (i: number, j: number) => T
  ) {
    this.rows = rows;
    this.columns = columns;
    this.items = [];
    for (var i = 0; i < rows; i++) {
      this.items[i] = [];
      for (var j = 0; j < columns; j++) {
        this.items[i][j] = valueConstructor(i, j);
      }
    }
  }

  get(index: GridIndex): T {
    this.throwIfOutOfBounds(index);
    return this.items[index.row][index.column];
  }

  set(index: GridIndex, value: T) {
    this.throwIfOutOfBounds(index);
    this.items[index.row][index.column] = value;
  }

  size(): number {
    return this.rows * this.columns;
  }

  forEach(callback: (value: T, index: GridIndex) => void) {
    for (var i = 0; i < this.rows; i++) {
      for (var j = 0; j < this.columns; j++) {
        callback(this.items[i][j], new GridIndex(i, j));
      }
    }
  }

  findAll(predicate: (value: T, index: GridIndex) => boolean): T[] {
    let items = [];
    for (var i = 0; i < this.rows; i++) {
      for (var j = 0; j < this.columns; j++) {
        if (predicate(this.items[i][j], new GridIndex(i, j))) {
          items.push(this.items[i][j]);
        }
      }
    }
    return items;
  }

  findAllIndices(predicate: (value: T, index: GridIndex) => boolean): GridIndex[] {
    let items: GridIndex[] = [];
    for (var i = 0; i < this.rows; i++) {
      for (var j = 0; j < this.columns; j++) {
        if (predicate(this.items[i][j], new GridIndex(i, j))) {
          items.push(new GridIndex(i,j));
        }
      }
    }
    return items;
  }


  isOutOfBounds(index: GridIndex): boolean {
    return (
      index.row < 0 ||
      index.row >= this.rows ||
      index.column < 0 ||
      index.column >= this.columns
    );
  }

  throwIfOutOfBounds(index: GridIndex) {
    if (this.isOutOfBounds(index)) {
      throw `Index ${index} is out of bounds`;
    }
  }

  toString(): string {
    let result = "";

    for (var i = 0; i < this.rows; i++) {
      if (i !== 0) {
        result += "\n";
      }
      for (var j = 0; j < this.columns; j++) {
        if (j !== 0) {
          result += " ";
        }
        result += this.items[i][j];
      }
    }
    return result;
  }
}

export class GridIndex {
  row: number;
  column: number;

  constructor(row: number, column: number) {
    this.row = row;
    this.column = column;
  }

  toString(): string {
    return `(${this.row},${this.column})`;
  }

  static add(lhs: GridIndex, rhs: GridIndex): GridIndex {
    return new GridIndex(lhs.row + rhs.row, lhs.column + rhs.column);
  }

  static neighbors<T>(index: GridIndex, grid: Grid<T>): GridIndex[] {
    var result: GridIndex[] = [];
    for (var offsetX = -1; offsetX <= 1; offsetX++) {
      for (var offsetY = -1; offsetY <= 1; offsetY++) {
        if (offsetX === 0 && offsetY === 0) {
          continue;
        }

        var neighborIndex = new GridIndex(
          index.row + offsetX,
          index.column + offsetY
        );

        if (grid.isOutOfBounds(neighborIndex)) {
          continue;
        }

        result.push(neighborIndex);
      }
    }
    return result;
  }
}
