/* eslint-disable no-unused-vars */
/* eslint-disable require-jsdoc */
import { read_input } from "./io";
import { Direction, Move, parse_move } from "./move";

class Coordinate {
  x = 0;
  y = 0;
}

function find_position(moves: Move[]): Coordinate {
  const position = new Coordinate();

  for (const move of moves) {
    switch (move.direction) {
      case Direction.Up:
        position.y -= move.amount;
        break;
      case Direction.Down:
        position.y += move.amount;
        break;
      case Direction.Forward:
        position.x += move.amount;
        break;
    }
  }

  return position;
}

const moves = read_input("input", parse_move);
const position = find_position(moves);

console.log(`Position: (${position.x},${position.y})`);
console.log(`Multiplied: ${position.x * position.y}`);
