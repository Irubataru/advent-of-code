import { readAges } from "./io";

var fish = readAges("input");
let fishAges = new Array<number>(9);

for (var i = 0; i < fishAges.length; i++) {
  fishAges[i] = fish.filter((age) => age === i).length;
}

for (var day = 0; day < 256; day++) {
  var readyFish = fishAges[0];
  for (var i = 1; i < fishAges.length; i++) {
    fishAges[i - 1] = fishAges[i];
  }

  fishAges[6] += readyFish;
  fishAges[8] = readyFish;
}

console.log(fishAges.reduce((total, next) => total + next, 0));
