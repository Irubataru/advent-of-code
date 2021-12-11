import { ParType } from "./parantheses"

export function getScore(type: ParType): number {
  switch (type) {
    case ParType.Paranthesis:
      return 3;
    case ParType.Bracket:
      return 57
    case ParType.Brace:
      return 1197;
    case ParType.Angle:
      return 25137;
  }
}
