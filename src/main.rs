extern crate rprompt;

fn move_eq (move1: &str, move2: &str) -> bool {
  return is_move(move1) && move1 == move2;
}

fn check_wins (board: [&str; 9]) -> bool {
    let wins = [
        [0, 1, 2],
        [0, 4, 8],
        [0, 3, 6],
        [2, 4, 6],
        [2, 5, 8],
        [3, 4, 5],
        [6, 7, 8],
        [1, 4, 7]
    ];

    for win in wins.iter() {
      if move_eq(board[win[0]], board[win[1]]) && move_eq(board[win[1]], board[win[2]]) {
        return true;
      }
    }
    return false;
}

fn print_board (board: [&str; 9]) {
    println!("
      {0} | {1} | {2}
      --------- 
      {3} | {4} | {5} 
      ---------
      {6} | {7} | {8}
    ", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
}

fn prompt_move (user: &str) -> i32 {
  let prompt = format!("{} move (1-9): ", user);
  let reply = rprompt::prompt_reply_stdout(&prompt).unwrap();
  let parsed:i32 = reply.parse::<i32>().unwrap() - 1;
  if parsed < 1 {
     return 0
  }
  return parsed;
}

fn is_move (maybe_move: &str) -> bool {
  return maybe_move == "X" || maybe_move == "O"
}

fn main() {
    let mut turn = "X";  
    let mut board: [&str; 9] = [" "; 9];
    let mut total_moves = 0;
    loop {
      print_board(board);
      let index = prompt_move(turn) as usize;

      if index > 8 {
        println!("You must pick a number between 1 and 9!");
        continue;
      } 

      if is_move(board[index])  {
        println!("Move invalid, pick an open space!");
        continue;
      }

      board[index] = turn;
      total_moves += 1;

      if check_wins(board) {
        print_board(board);
        println!("Game over! {} wins", board[index]);
        break;
      }

      if total_moves == 9 {
        println!("Game over! Stalemate.");
        break;
      }

      if turn == "X" {
        turn = "O"
      } else {
        turn = "X"
      } 
    }
}