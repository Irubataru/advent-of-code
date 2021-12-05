import { BingoBoard } from "./board"

export function play_to_win(draws: number[], boards: BingoBoard[]): void {
  for (var draw of draws) {
    for (var board of boards) {
      board.register_new_draw(draw);

      if (board.has_won()) {
        var score = board.get_score();

        console.log("The following board has won!");
        console.log(`${board}`);

        console.log("");
        console.log(`Winning draw: ${draw}`);
        console.log(`Board score: ${score}`);
        console.log(`Puzzle solution: ${draw * score}`);

        return;
      }
    }
  }
}

export function play_to_lose(draws: number[], boards: BingoBoard[]): void {

  // Keep track of the boards that have already won
  let completed_boards: number[] = [];

  for (var draw of draws) {

    for (var bidx = 0; bidx < boards.length; bidx++) {

      if (completed_boards.includes(bidx)) { continue; }

      let board = boards[bidx];
      board.register_new_draw(draw);

      if (board.has_won()) {

        var score = board.get_score();

        if (completed_boards.length < boards.length - 1) {
          completed_boards.push(bidx);
          continue;
        }

        console.log("The following board has lost!");
        console.log(`${board}`);

        console.log("");
        console.log(`Final draw: ${draw}`);
        console.log(`Board score: ${score}`);
        console.log(`Puzzle solution: ${draw * score}`);

        return;
      }
    }
  }
}
