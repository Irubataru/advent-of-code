import { read_input } from "./io";
import { parse_bits, get_oxygen_rate, get_co2_rate } from "./bits";

var bits = read_input("input", parse_bits);

const oxygen_rate = get_oxygen_rate(bits);
const co2_rate = get_co2_rate(bits);

console.log(`Oxygen generator rate: ${oxygen_rate}`);
console.log(`CO2 scrubbing rate: ${co2_rate}`);
console.log(`Life support rate: ${oxygen_rate * co2_rate}`);
