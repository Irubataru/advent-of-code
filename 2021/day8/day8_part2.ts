import { readInput, Input } from "./io";
import { Display, DisplayValue } from "./display";

function only<T>(values: T[]): T {
  if (values.length !== 1) {
    throw `Expected array of 1 element, got ${values.length}`;
  }

  return values[0];
}

function decode(input: Input): number {
  var encoding = new Array<DisplayValue>(10);
  encoding[1] = only(input.patterns.filter((i) => i.segments.length == 2));
  encoding[4] = only(input.patterns.filter((i) => i.segments.length == 4));
  encoding[7] = only(input.patterns.filter((i) => i.segments.length == 3));
  encoding[8] = only(input.patterns.filter((i) => i.segments.length == 7));

  encoding[9] = only(
    input.patterns.filter(
      (i) => i.segments.length == 6 && i.intersects(encoding[4])
    )
  );
  encoding[0] = only(
    input.patterns.filter(
      (i) =>
        i.segments.length == 6 && i != encoding[9] && i.intersects(encoding[7])
    )
  );
  encoding[6] = only(
    input.patterns.filter(
      (i) => i.segments.length == 6 && i != encoding[9] && i != encoding[0]
    )
  );

  encoding[3] = only(
    input.patterns.filter(
      (i) => i.segments.length == 5 && i.intersects(encoding[7])
    )
  );
  encoding[5] = only(
    input.patterns.filter(
      (i) =>
        i.segments.length == 5 && i != encoding[3] && encoding[9].intersects(i)
    )
  );
  encoding[2] = only(
    input.patterns.filter(
      (i) => i.segments.length == 5 && i != encoding[3] && i != encoding[5]
    )
  );

  input.displays.reverse();

  var result = 0;
  for (var index = 0; index < input.displays.length; index++) {
    // console.log(`${i} -> ${input.displays[i]}`);
    result += encoding.reduce<number>(
      (v, e, i) => v + (e.match(input.displays[index]) ? i : 0) * Math.pow(10, index),
      0
    );
    encoding.reduce;
  }

  return result;
}

var inputs = readInput("input");

console.log(inputs.reduce<number>((n, i) => n + decode(i), 0));
