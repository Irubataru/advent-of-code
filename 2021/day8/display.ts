export class DisplayValue {
  segments: number[];

  constructor(segments: number[]) {
    this.segments = segments;
  }

  toString(): string {
    return `[${this.segments}]`;
  }

  intersects(other: DisplayValue): boolean {
    return other.segments.every((o) => this.segments.includes(o));
  }

  match(other: DisplayValue): boolean {
    if (this.segments.length !== other.segments.length) {
      return false;
    }
    return this.intersects(other);
  }
}
