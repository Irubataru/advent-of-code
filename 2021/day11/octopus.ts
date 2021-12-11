import { Grid, GridIndex } from "../common/grid"

export class Octopus {
  energy: number = 0;
  constructor(energy: number) {
    this.energy = energy;
  }

  toString(): string {
    return this.energy.toString();
  }
}

export function flash(octopodes: Grid<Octopus>): number {
  octopodes.forEach((oct) => oct.energy++);

  let flashes = 0;
  while(true) {
    const flashingIndex = octopodes.findAllIndices((oct) => oct.energy > 9);

    if (flashingIndex.length === 0) {
      break;
    }

    flashes += flashingIndex.length;

    for (var index of flashingIndex) {
      var neighbors = GridIndex.neighbors(index, octopodes);

      let oct = octopodes.get(index);
      oct.energy = 0;

      for (var neighborIndex of neighbors) {
        let oct = octopodes.get(neighborIndex);
        if (oct.energy !== 0) {
          oct.energy += 1;
        }
      }
    }
  }

  return flashes;
}
