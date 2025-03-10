#[allow(unused, unused_imports)]
pub mod sttt;
mod monte_carlo;

use std::time::Instant;
use std::io;
use sttt::{ttt, StrategicBoard};
use ttt::{GameState, MoveResult};
use monte_carlo::*;

// fn read(prompt: &str) -> usize {
//     println!("{}", prompt);

//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     match input.trim().parse::<usize>() {
//         Ok(num) if num >= 1 && num <= 9 => num,
//         _ => {
//             println!("Invalid input. Please enter a number between 1 and 9.");
//             0
//         }
//     }
// }

fn main() {
    let mut new_board = StrategicBoard::new();
    let mut new_tree = Tree::new();
    for _ in 0..10 {
        new_tree.step(&mut new_board);
    }
    println!("{:?}", &new_tree.root);
    println!("{:?}", new_board);
    
    // new_board.set_checkpoint();

    // let now = Instant::now();
    // let count = 100_000;
    
    // for game in 0..count {
    //     new_board.revert();
        
    //     if game % 10000 == 0 {
    //         println!("{}% of games completed", (game as f32 / count as f32) * 100.0);
    //     }

    //     loop {
    //         let new_move = new_board.get_random_move();
    //         let result = new_board.make_move(new_move.subboard, new_move.index);

    //         match result {
    //             MoveResult::Completed(p) => {
    //                 break
    //             }
    //             _ => ()
    //         };
    //     }
    // }

    // println!("Was able to compute {:?} games per second", count as f64 / now.elapsed().as_secs_f64());

    // loop {
    //     let subboard = match new_board.current_board {
    //         Some(x) => {
    //             println!("Currently on board {}", x + 1);
    //             x
    //         }
    //         None => read("Subboard (1-9): ") - 1,
    //     };
    //     let index = read("Index (1-9): ") - 1;

    //     let result = new_board.make_move(subboard, index);
    //     new_board.display();
    //     println!("{:?}", new_board.get_random_move());

    //     match result {
    //         MoveResult::Completed(p) => {
    //             match p {
    //                 1 => println!("Player 1 won!"),
    //                 -1 => println!("Player 2 won!"),
    //                 2 => println!("Game was a draw"),
    //                 _ => unreachable!(),
    //             };
    //             break;
    //         }
    //         MoveResult::Nothing => (),
    //     }
    // }
}
