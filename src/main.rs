use std::io::{self, Read};
use Players::Player;

mod Deck;
mod Draw;
mod Players;

fn start_game(mut deck: Deck::Deck, player: &mut Players::Player, dealer: &mut Players::Player) {
    for _ in 0..2 {
        dealer.push(deck.get_top_card());
        player.push(deck.get_top_card());
    }
}

enum Move {
    Hit,
    Stand,
}

#[derive(Debug)]
enum Error {
    ParsingError,
}

fn parse_move() -> Result<Move, Error> {
    let mut choice = String::new();

    println!("What would you like to do?");
    println!("H: Hit    S: Stand");

    io::stdin()
        .read_line(&mut choice)
        .expect("Unable to read input");

    match choice.trim() {
        "H" => Ok(Move::Hit),
        "S" => Ok(Move::Stand),
        _ => parse_move(),
    }
}

fn main() {
    let mut player = &mut Player::new();
    let mut dealer = &mut Player::new();
    let deck = Deck::Deck::new();
    start_game(deck, &mut player, &mut dealer);
    loop {
        Draw::draw(player, dealer);
        match parse_move() {
            Ok(input) => match input {
                Move::Hit => {
                    println!("Hitting");
                }
                Move::Stand => {
                    println!("Standing");
                }
            },
            Err(_) => panic!("This should never happen"),
        }
    }
}
