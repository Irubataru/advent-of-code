export enum ParDirection {
  Open,
  Close,
}

export enum ParType {
  Paranthesis,
  Bracket,
  Brace,
  Angle,
}

export class Para {
  type: ParType;
  dir: ParDirection;

  constructor(type: ParType, dir: ParDirection) {
    this.type = type;
    this.dir = dir;
  }

  toString(): string {
    switch (this.type) {
      case ParType.Paranthesis:
        return this.dir === ParDirection.Open ? "(" : ")";
      case ParType.Bracket:
        return this.dir === ParDirection.Open ? "[" : "]";
      case ParType.Brace:
        return this.dir === ParDirection.Open ? "{" : "}";
      case ParType.Angle:
        return this.dir === ParDirection.Open ? "<" : ">";
    }
  }
}

export function parsePara(str: string): Para {
  switch (str) {
    case "(":
      return new Para(ParType.Paranthesis, ParDirection.Open);
    case ")":
      return new Para(ParType.Paranthesis, ParDirection.Close);
    case "[":
      return new Para(ParType.Bracket, ParDirection.Open);
    case "]":
      return new Para(ParType.Bracket, ParDirection.Close);
    case "{":
      return new Para(ParType.Brace, ParDirection.Open);
    case "}":
      return new Para(ParType.Brace, ParDirection.Close);
    case "<":
      return new Para(ParType.Angle, ParDirection.Open);
    case ">":
      return new Para(ParType.Angle, ParDirection.Close);
    default:
      throw `parsePara error: '${str}' is not a valid paranthesis`;
  }
}
