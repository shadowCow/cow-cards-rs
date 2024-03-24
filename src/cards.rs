use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: vec![
                Card { suit: Suit::Clubs, rank: Rank::Ace },
                Card { suit: Suit::Clubs, rank: Rank::Two },
                Card { suit: Suit::Clubs, rank: Rank::Three },
                Card { suit: Suit::Clubs, rank: Rank::Four },
                Card { suit: Suit::Clubs, rank: Rank::Five },
                Card { suit: Suit::Clubs, rank: Rank::Six },
                Card { suit: Suit::Clubs, rank: Rank::Seven },
                Card { suit: Suit::Clubs, rank: Rank::Eight },
                Card { suit: Suit::Clubs, rank: Rank::Nine },
                Card { suit: Suit::Clubs, rank: Rank::Ten },
                Card { suit: Suit::Clubs, rank: Rank::Jack },
                Card { suit: Suit::Clubs, rank: Rank::Queen },
                Card { suit: Suit::Clubs, rank: Rank::King },
                Card { suit: Suit::Diamonds, rank: Rank::Ace },
                Card { suit: Suit::Diamonds, rank: Rank::Two },
                Card { suit: Suit::Diamonds, rank: Rank::Three },
                Card { suit: Suit::Diamonds, rank: Rank::Four },
                Card { suit: Suit::Diamonds, rank: Rank::Five },
                Card { suit: Suit::Diamonds, rank: Rank::Six },
                Card { suit: Suit::Diamonds, rank: Rank::Seven },
                Card { suit: Suit::Diamonds, rank: Rank::Eight },
                Card { suit: Suit::Diamonds, rank: Rank::Nine },
                Card { suit: Suit::Diamonds, rank: Rank::Ten },
                Card { suit: Suit::Diamonds, rank: Rank::Jack },
                Card { suit: Suit::Diamonds, rank: Rank::Queen },
                Card { suit: Suit::Diamonds, rank: Rank::King },
                Card { suit: Suit::Hearts, rank: Rank::Ace },
                Card { suit: Suit::Hearts, rank: Rank::Two },
                Card { suit: Suit::Hearts, rank: Rank::Three },
                Card { suit: Suit::Hearts, rank: Rank::Four },
                Card { suit: Suit::Hearts, rank: Rank::Five },
                Card { suit: Suit::Hearts, rank: Rank::Six },
                Card { suit: Suit::Hearts, rank: Rank::Seven },
                Card { suit: Suit::Hearts, rank: Rank::Eight },
                Card { suit: Suit::Hearts, rank: Rank::Nine },
                Card { suit: Suit::Hearts, rank: Rank::Ten },
                Card { suit: Suit::Hearts, rank: Rank::Jack },
                Card { suit: Suit::Hearts, rank: Rank::Queen },
                Card { suit: Suit::Hearts, rank: Rank::King },
                Card { suit: Suit::Spades, rank: Rank::Ace },
                Card { suit: Suit::Spades, rank: Rank::Two },
                Card { suit: Suit::Spades, rank: Rank::Three },
                Card { suit: Suit::Spades, rank: Rank::Four },
                Card { suit: Suit::Spades, rank: Rank::Five },
                Card { suit: Suit::Spades, rank: Rank::Six },
                Card { suit: Suit::Spades, rank: Rank::Seven },
                Card { suit: Suit::Spades, rank: Rank::Eight },
                Card { suit: Suit::Spades, rank: Rank::Nine },
                Card { suit: Suit::Spades, rank: Rank::Ten },
                Card { suit: Suit::Spades, rank: Rank::Jack },
                Card { suit: Suit::Spades, rank: Rank::Queen },
                Card { suit: Suit::Spades, rank: Rank::King },
            ]
        }
    }

    pub fn shuffle(&self) -> Deck {
        let mut deck_copy = self.clone();
        deck_copy.shuffle_mut();
        
        deck_copy
    }

    pub fn shuffle_mut(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&self) -> (Option<Card>, Deck) {
        let mut deck_copy = self.clone();

        let card = deck_copy.cards.pop();

        (card, deck_copy)
    }

    pub fn draw_mut(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {

    }

    #[test]
    fn test_draw() {

    }
}