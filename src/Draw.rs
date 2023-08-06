use crate::Players::Player;

use super::Deck;
use super::Players;

pub fn draw(player: &mut Player, dealer: &mut Player) {
    println!("Dealers Cards: ");
    for card in dealer.cards.clone() {
        println!("  {:?}", card);
    }

    println!("Your Cards: ");
    let mut player_total = 0;
    for card in player.cards.clone() {
        player_total += card.get_value();
        println!("  {:?}", card);
    }
    println!("  Your Total: {:?}", player_total)
}
