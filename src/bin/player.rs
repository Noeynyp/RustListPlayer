use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let player1 = args.get(1).cloned().unwrap_or(String::from("N/A"));
    let player2 = args.get(2).cloned().unwrap_or(String::from("N/A"));

    println!("Player 1: {}", player1);
    println!("Player 2: {}", player2);
}
