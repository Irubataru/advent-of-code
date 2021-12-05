import { read_input } from "./io";
import { parse_bits, get_gamma_rate, get_epsilon_rate } from "./bits";

var bits = read_input("input", parse_bits);

const gamma_rate = get_gamma_rate(bits);
const epsilon_rate = get_epsilon_rate(bits);

console.log(`Gamma rate: ${gamma_rate}`);
console.log(`Epsilon rate ${epsilon_rate}`);
console.log(`Power consumption: ${gamma_rate * epsilon_rate}`);
