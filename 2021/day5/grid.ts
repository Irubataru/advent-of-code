export class Grid<T> {
  private items: T[][];
  private size: number;

  constructor(size: number, init: T) {
    this.size = size;
    this.items = [];
    for (var i = 0; i < size; i++) {
      this.items[i] = [];
      for (var j = 0; j < size; j++) {
        this.items[i][j] = init;
      }
    }
  }

  getItem(i: number, j: number): T {
    this.throw_if_out_of_bounds(i, j);
    return this.items[i][j];
  }

  setItem(i: number, j: number, value: T): void {
    this.throw_if_out_of_bounds(i, j);
    this.items[i][j] = value;
  }

  getSize(): number {
    return this.size;
  }

  private throw_if_out_of_bounds(i: number, j: number): void {
    if (i < 0 || i >= this.size || j < 0 || j >= this.size) {
      throw `Out of bounds: ${i} and ${j}`;
    }
  }
}
