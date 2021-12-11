import { readInput } from "./io"
import { Stack } from "./stack"
import { ParDirection, ParType, Para, parsePara } from "./parantheses"
import { getScore } from "./scoring"

let input = readInput("input");

var score = 0;

for (var line of input) {

  let stack = new Stack<ParType>();

  for (var i = 0; i < line.length; i++) {

    var par = parsePara(line[i]);
    if (par.dir === ParDirection.Open) {
      stack.push(par.type);
      continue;
    }

    var toClose = stack.peek();

    if (toClose != par.type) {
      console.log(`Expected '${new Para(toClose, ParDirection.Close)}', but found '${par}' instead.`);
      score += getScore(par.type);
      stack.drop();
      break;
    }

    stack.pop();
  }
}

console.log(`Final score: ${score}.`)
