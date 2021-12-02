import * as fs from 'fs'

function read_input(filename: string): number[] {
  var lines = fs.readFileSync(filename, 'utf8').split(/\r?\n/).slice(0, -1);
  return lines.map(function(line: string) { return parseInt(line); });
}

function count_depth_increases(depths: number[], window: number) {

  var previous_depth = Number.MAX_VALUE;

  var increases: number = 0;
  for (var i = 0; i < depths.length - window + 1; i++) {

    var current_depth = depths
      .slice(i + 1, i + window)
      .reduce(
        function(a: number, b: number) {
          return a + b;
        }, depths[i]);

    if (current_depth > previous_depth) {
      increases += 1;
    }

    previous_depth = current_depth;
  }

  return increases;
}

var depths = read_input('input.dat');

console.log(count_depth_increases(depths, 1));
console.log(count_depth_increases(depths, 3));
