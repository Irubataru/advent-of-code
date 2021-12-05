export enum Direction {
  Forward,
  Up,
  Down,
  None,
}

export class Move {
  direction: Direction = Direction.None;
  amount = 0;

  constructor(direction: Direction, amount: number) {
    this.direction = direction;
    this.amount = amount;
  }
}

export function parse_direction(value: string): Direction {
  switch (value) {
    case "forward":
      return Direction.Forward;
    case "up":
      return Direction.Up;
    case "down":
      return Direction.Down;
    default:
      return Direction.None;
  }
}

export function parse_move(line: string): Move {
  const args = line.split(" ");
  return new Move(parse_direction(args[0]), parseInt(args[1]));
}
