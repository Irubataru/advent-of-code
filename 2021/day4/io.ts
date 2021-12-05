import * as fs from "fs";
import { BingoBoard } from "./board";

function get_bingo_draws(line: string): number[] {
  return line.split(",").map((value) => parseInt(value));
}

function get_bingo_line(line: string): number[] {
  return line
    .trim()
    .split(/\s+/)
    .map((value) => parseInt(value));
}

function get_bingo_board(lines: string[], row: number): [number, BingoBoard] {
  var size = get_bingo_line(lines[row]).length;
  var board = new BingoBoard(size);

  for (var i = 0; i < size; i++) {
    var bingo_row = get_bingo_line(lines[row + i]);

    for (var j = 0; j < size; j++) {
      board.set(i, j, bingo_row[j]);
    }
  }

  return [row + size, board];
}

export function get_game_data(filename: string): [number[], BingoBoard[]] {
  const lines = fs.readFileSync("input", "utf8").split(/\r?\n/).slice(0, -1);

  const draws = get_bingo_draws(lines[0]);

  let boards: BingoBoard[] = [];

  let row = 1;
  while (row < lines.length) {

    // Skip empty lines
    if (lines[row].length == 0) {
      row += 1;
      continue;
    }

    let board: BingoBoard;
    [row, board] = get_bingo_board(lines, row);

    boards.push(board);
  }

  return [draws, boards];
}
