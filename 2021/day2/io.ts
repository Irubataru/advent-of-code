import * as fs from "fs";

export function read_input<T>(filename: string, transform: (s: string) => T): T[] {
  const lines = fs.readFileSync(filename, "utf8").split(/\r?\n/).slice(0, -1);
  return lines.map(transform);
}
