import { readInput } from "./io";
import { Stack } from "./stack";
import { ParDirection, ParType, parsePara } from "./parantheses";
import { getScore } from "./scoring2";

let input = readInput("input");

var scores: number[] = [];

for (var line of input) {
  let stack = new Stack<ParType>();

  let isValid = true;
  for (var i = 0; i < line.length; i++) {
    var par = parsePara(line[i]);
    if (par.dir === ParDirection.Open) {
      stack.push(par.type);
      continue;
    }

    var toClose = stack.peek();

    if (toClose != par.type) {
      isValid = false;
      break;
    }

    stack.pop();
  }

  if (!isValid) {
    continue;
  }

  let score = 0;
  while (!stack.empty()) {
    score = score * 5 + getScore(stack.pop());
  }

  scores.push(score);
}

scores.sort((a,b) => a - b);
console.log(`Winning score is ${scores[Math.floor(scores.length / 2)]}`)
