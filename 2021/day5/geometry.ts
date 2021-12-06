export class Coordinate {
  x: number = 0;
  y: number = 0;
}

export class Line {
  point: Coordinate = new Coordinate();
  vector: Coordinate = new Coordinate();
}
