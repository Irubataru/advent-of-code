import { readInput } from "./io"

var inputs = readInput("input");

var counts: number[] = new Array<number>(10).fill(0);
for (var input of inputs) {
  for (var disp of input.displays) {
    counts[1] += (disp.segments.length == 2 ? 1 :0);
    counts[4] += (disp.segments.length == 4 ? 1 :0);
    counts[7] += (disp.segments.length == 3 ? 1 :0);
    counts[8] += (disp.segments.length == 7 ? 1 :0);
  }
}

console.log(counts[1] + counts[4] + counts[7] + counts[8]);
