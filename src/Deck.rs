use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Rank {
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Ace,
}

pub struct Deck {
    pub deck: VecDeque<Card>,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: VecDeque<Card> = VecDeque::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                let suit = suit.clone();
                let card = Card { rank, suit };
                deck.push_back(card);
            }
        }
        let mut deck = Deck { deck };
        deck.shuffle();
        deck
    }

    pub fn debug(&self) {
        for card in &self.deck {
            println!("{:?}", card);
        }
    }

    pub fn shuffle(&mut self) {
        let mut deck = Vec::from(self.deck.clone());
        deck.shuffle(&mut thread_rng());
        self.deck = VecDeque::from(deck);
    }

    pub fn get_top_card(&mut self) -> Card {
        let first = self.deck.front().unwrap().clone();
        self.deck.pop_front();
        first
    }
}

impl Card {
    pub fn get_value(&self) -> u8 {
        match self.rank {
            Rank::King => 10,
            Rank::Queen => 10,
            Rank::Jack => 10,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
            Rank::Ace => 1,
        }
    }
}
