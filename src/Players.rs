use crate::Deck::Card;
use crate::Deck::Deck;

pub struct Player {
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new() -> Player {
        Player { cards: Vec::new() }
    }
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn calc_score(&self) {
        let mut total = 0;
        for card in &self.cards {
            total += card.get_value();
        }
    }
}
