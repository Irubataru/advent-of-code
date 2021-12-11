import * as io from "../common/io";
import { Octopus, flash } from "./octopus";
import { Grid } from "../common/grid";

var energyLevels = io.readIntegerGrid("input", "");

var octopodes = new Grid<Octopus>(
  energyLevels.length,
  energyLevels[0].length,
  (i, j) => new Octopus(energyLevels[i][j])
);

let step = 1;
while (true) {
  if (flash(octopodes) === octopodes.size()) {
    break;
  }
  step++;
}

console.log(`They all flash at step ${step}`)
