import { ParType } from "./parantheses"

export function getScore(type: ParType): number {
  switch (type) {
    case ParType.Paranthesis:
      return 1;
    case ParType.Bracket:
      return 2
    case ParType.Brace:
      return 3;
    case ParType.Angle:
      return 4;
  }
}
