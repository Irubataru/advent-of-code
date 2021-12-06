import * as fs from "fs";
import { Coordinate, Line } from "./geometry";

function parseCoordinate(value: string): Coordinate {
  var [lhs, rhs] = value.split(",");

  var coordinate = new Coordinate();
  coordinate.x = parseInt(lhs);
  coordinate.y = parseInt(rhs);

  return coordinate;
}

function parseLine(value: string): Line {
  var line = new Line();

  var [lhs, rhs] = value.split(" -> ");

  line.point = parseCoordinate(lhs);

  var to = parseCoordinate(rhs);

  line.vector.x = to.x - line.point.x;
  line.vector.y = to.y - line.point.y;

  return line;
}

export function readLines(filename: string): Line[] {
  return fs
    .readFileSync(filename, "utf8")
    .toString()
    .split("\n")
    .filter((x) => x.length)
    .map(parseLine);
}
