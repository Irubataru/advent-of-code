export class BingoBoard {
  private items: number[][];
  private hits: boolean[][];
  private size: number;

  constructor(size: number) {
    this.size = size;

    this.items = [];
    this.hits = [];
    for (var i = 0; i < size; i++) {
      this.items[i] = [];
      this.hits[i] = [];
      for (var j = 0; j < size; j++) {
        this.items[i][j] = 0;
        this.hits[i][j] = false;
      }
    }
  }

  get(i: number, j: number): number {
    this.throw_if_out_of_bounds(i, j);
    return this.items[i][j];
  }

  set(i: number, j: number, value: number): void {
    this.throw_if_out_of_bounds(i, j);
    this.items[i][j] = value;
  }

  register_new_draw(value: number) {
    for (var i = 0; i < this.size; i++) {
      for (var j = 0; j < this.size; j++) {
        if (this.items[i][j] == value) {
          this.hits[i][j] = true;
        }
      }
    }
  }

  has_won(): boolean {
    if (this.has_complete_row()) {
      return true;
    }

    return this.has_complete_column();
  }

  get_score(): number {
    let score = 0;
    for (var i = 0; i < this.size; i++) {
      for (var j = 0; j < this.size; j++) {
        if (!this.hits[i][j]) {
          score += this.items[i][j];
        }
      }
    }

    return score;
  }

  toString(): string {
    var str: string = "";
    for (var i = 0; i < this.size; i++) {
      for (var j = 0; j < this.size; j++) {
        str += `${String(this.items[i][j]).padStart(2, " ")} `;
        str += `(${this.hits[i][j] ? "x" : " "})`;
        if (j < this.size - 1) {
          str += "  ";
        }
      }
      if (i < this.size - 1) {
        str += "\n";
      }
    }

    return str;
  }

  private has_complete_row(): boolean {
    for (var row = 0; row < this.size; row++) {
      let row_match = true;
      for (var column = 0; column < this.size; column++) {
        if (!this.hits[row][column]) {
          row_match = false;
          break;
        }
      }

      if (row_match) {
        return true;
      }
    }

    return false;
  }

  private has_complete_column(): boolean {
    for (var column = 0; column < this.size; column++) {
      let column_match = true;
      for (var row = 0; row < this.size; row++) {
        if (!this.hits[row][column]) {
          column_match = false;
          break;
        }
      }

      if (column_match) {
        return true;
      }
    }

    return false;
  }

  private throw_if_out_of_bounds(i: number, j: number): void {
    if (i < 0 || i >= this.size || j < 0 || j >= this.size) {
      throw `Out of bounds: ${i} and ${j}`;
    }
  }
}
