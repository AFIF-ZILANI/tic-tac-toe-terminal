use std::io;

fn main() {
    let mut main_array: [[&str; 3]; 3] = [[" ", " ", " "], [" ", " ", " "], [" ", " ", " "]];
    let mut turn = "X";
    loop {
        if turn == "X" {
            let position: [usize; 2] = user_input(&main_array, &turn);
            if main_array[position[0]][position[1]] == "X" {
                println!("This move already occured!, try another move!");
                continue;
            };
            main_array[position[0]][position[1]] = "X";
            let winner = check_winner("X", main_array);
            if winner == true {
                board(&main_array);
                println!("Player X is winner");
                break;
            }
            turn = "O";
        } else {
            let position: [usize; 2] = user_input(&main_array, &turn);
            if main_array[position[0]][position[1]] == "O" {
                println!("This move already occured!, try another move!");
                continue;
            };
            main_array[position[0]][position[1]] = "O";
            let winner = check_winner("O", main_array);
            if winner == true {
                board(&main_array);
                println!("Player O is winner");
                break;
            }
            turn = "X";
        };
        if !main_array[0].contains(&" ")
            && !main_array[1].contains(&" ")
            && !main_array[2].contains(&" ")
        {
            board(&main_array);
            println!("The match has tie!");
            break;
        }
    }
}

fn user_input(main_array: &[[&str; 3]; 3], turn: &str) -> [usize; 2] {
    let mut raw_input = String::new();

    board(main_array);
    println!("Turn of {}", turn);
    println!("Enter your next position (e.g., 00, 01, etc.):");

    io::stdin()
        .read_line(&mut raw_input)
        .expect("Unable to read user input");

    let input: &str = raw_input.trim();
    let position = match input {
        "00" => [0, 0],
        "01" => [0, 1],
        "02" => [0, 2],
        "10" => [1, 0],
        "11" => [1, 1],
        "12" => [1, 2],
        "20" => [2, 0],
        "21" => [2, 1],
        "22" => [2, 2],
        _ => {
            println!("Invalid position! Please enter a valid position.");
            return user_input(main_array, turn);
        }
    };

    position
}

fn board(board_array: &[[&str; 3]; 3]) {
    println!(
        "
                 |            |
                 |            |
           {}     |      {}     |      {}
                 |            |
     --------------------------------------
                 |            |
                 |            |
           {}     |      {}     |      {}
                 |            |
                 |            |
     --------------------------------------
                 |            |
                 |            |
           {}     |      {}     |      {}
                 |            |
                 |            |
    ",
        board_array[0][0],
        board_array[0][1],
        board_array[0][2],
        board_array[1][0],
        board_array[1][1],
        board_array[1][2],
        board_array[2][0],
        board_array[2][1],
        board_array[2][2],
    )
}

fn check_winner(player: &str, main_array: [[&str; 3]; 3]) -> bool {
    if ((main_array[0][0] == player)
        && (main_array[0][1] == player)
        && (main_array[0][2] == player))
        || ((main_array[1][0] == player)
            && (main_array[1][1] == player)
            && (main_array[1][2] == player))
        || ((main_array[2][0] == player)
            && (main_array[2][1] == player)
            && (main_array[2][2] == player))
        || ((main_array[0][0] == player)
            && (main_array[1][0] == player)
            && (main_array[2][0] == player))
        || ((main_array[0][1] == player)
            && (main_array[1][1] == player)
            && (main_array[2][1] == player))
        || ((main_array[0][2] == player)
            && (main_array[1][2] == player)
            && (main_array[2][2] == player))
        || ((main_array[0][0] == player)
            && (main_array[1][1] == player)
            && (main_array[2][2] == player))
        || ((main_array[0][2] == player)
            && (main_array[1][1] == player)
            && (main_array[2][0] == player))
    {
        return true;
    } else {
        return false;
    }
}



//
//
// Board Desin
//          0              1            2
//                 |            |
//                 |            |
//  0        X     |      X     |      X
//                 |            |
//     --------------------------------------
//                 |            |
//                 |            |
//  1        X     |      X     |      X
//                 |            |
//                 |            |
//     --------------------------------------
//                 |            |
//                 |            |
//  2        X     |      X     |      X
//                 |            |
//                 |            |
