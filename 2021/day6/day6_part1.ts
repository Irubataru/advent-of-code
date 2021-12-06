import { readFish } from "./io";
import { LanternFish } from "./fish";

let fishes = readFish("input");

for (var day = 0; day < 80; day++) {
  fishes = [
    ...fishes,
    ...(fishes
      .map((f) => f.nextDay())
      .filter((x) => x !== null) as LanternFish[]),
  ];
}

console.log(fishes.length);
