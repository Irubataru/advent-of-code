import { get_game_data } from "./io"
import { play_to_win } from "./game"

let [draws, boards] = get_game_data("input");
play_to_win(draws, boards);
