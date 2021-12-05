export type Bits = Array<number>;

export function parse_bits(line: string): Bits {
  let result = new Array<number>();
  for (var i = 0; i < line.length; i++) {
    result.push(parseInt(line[i]));
  }

  return result;
}

function count_ones(bits: Array<Bits>, index: number): number {
  return bits.reduce((total, next) => (total += next[index]), 0);
}

function most_common_bit(bits: Array<Bits>, index: number): number {
  const one_bits = count_ones(bits, index);

  if (one_bits * 2 >= bits.length) {
    return 1;
  } else {
    return 0;
  }
}

function least_common_bit(bits: Array<Bits>, index: number): number {
  const one_bits = count_ones(bits, index);

  if (one_bits * 2 >= bits.length) {
    return 0;
  } else {
    return 1;
  }
}

function reduce_bits(
  bits: Array<Bits>,
  reducer: (b: Array<Bits>, i: number) => number
): Bits {
  const len = bits[0].length;

  var result = new Array<number>();
  for (var i = 0; i < len; i++) {
    result.push(reducer(bits, i));
  }

  return result;
}

function most_common_filter(bits: Array<Bits>, i: number): Array<Bits> {
  const one_bits = bits.reduce((total, next) => total + (next[i] ? 1 : 0), 0);

  if (one_bits * 2 >= bits.length) {
    return bits.filter((value) => value[i] === 1);
  } else {
    return bits.filter((value) => value[i] === 0);
  }
}

function least_common_filter(bits: Array<Bits>, i: number): Array<Bits> {
  const one_bits = bits.reduce((total, next) => total + (next[i] ? 1 : 0), 0);

  var least_common: number;
  if (one_bits * 2 === bits.length) {
    least_common = 0;
  } else if (one_bits * 2 > bits.length) {
    least_common = 0;
  } else {
    least_common = 1;
  }

  return bits.filter((value) => value[i] == least_common);
}

function filter_bits(
  bits: Array<Bits>,
  filter: (b: Array<Bits>, i: number) => Array<Bits>
): Bits {
  const len = bits[0].length;

  for (var i = 0; i < len; i++) {
    bits = filter(bits, i);
    if (bits.length === 1) {
      break;
    }
  }

  return bits[0];
}

function number_from_bits(bits: Bits): number {
  let result = 0;
  for (var i = 0; i < bits.length; i++) {
    result += bits[i] * (1 << i);
  }

  return result;
}

export function get_gamma_rate(bits: Array<Bits>) {
  return number_from_bits(reduce_bits(bits, most_common_bit).reverse());
}

export function get_epsilon_rate(bits: Array<Bits>) {
  return number_from_bits(reduce_bits(bits, least_common_bit).reverse());
}

export function get_oxygen_rate(bits: Array<Bits>) {
  return number_from_bits(filter_bits(bits, most_common_filter).reverse());
}

export function get_co2_rate(bits: Array<Bits>) {
  return number_from_bits(filter_bits(bits, least_common_filter).reverse());
}
