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

fn main() {
    let mut player = &mut Player::new();
    let mut dealer = &mut Player::new();
    let deck = Deck::Deck::new();
    start_game(deck, &mut player, &mut dealer);

    Draw::draw(player, dealer);
}
