/* eslint-disable no-unused-vars */
/* eslint-disable require-jsdoc */
import { read_input } from "./io";
import { Direction, Move, parse_move } from "./move";

class Coordinate {
  x = 0;
  y = 0;
}

function find_position_with_aim(moves: Move[]): Coordinate {

  const position = new Coordinate();

  let aim = 0;

  for (const move of moves) {
    switch (move.direction) {
      case Direction.Up:
        aim -= move.amount;
        break;
      case Direction.Down:
        aim += move.amount;
        break;
      case Direction.Forward:
        position.x += move.amount;
        position.y += move.amount * aim;
        break;
    }
  }

  return position;
}

const moves = read_input("input", parse_move);
const position = find_position_with_aim(moves);

console.log(`Position: (${position.x},${position.y})`);
console.log(`Multiplied: ${position.x * position.y}`);
