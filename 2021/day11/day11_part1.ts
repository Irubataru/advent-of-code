import * as io from "../common/io";
import { Octopus, flash } from "./octopus";
import { Grid } from "../common/grid";

var energyLevels = io.readIntegerGrid("input", "");

var octopodes = new Grid<Octopus>(
  energyLevels.length,
  energyLevels[0].length,
  (i, j) => new Octopus(energyLevels[i][j])
);

var flashes = 0;
for (var i = 0; i < 100; i++) {
  flashes += flash(octopodes);
}

console.log(`Total flashes ${flashes}`);
