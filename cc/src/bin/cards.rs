#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        use Card::*;

        let mut sum = 0usize;
        let mut ace_count = 0usize;

        self.cards.iter()
            .for_each(|card| {
               sum += match card {
                   Ace => {
                       ace_count += 1;
                       0
                   }
                   Two => 2,
                   Three => 3,
                   Four => 4,
                   Five => 5,
                   Six => 6,
                   Seven => 7,
                   Eight => 8,
                   Nine => 9,
                   Jack | Queen | King => 10
               }
            });

        (0..ace_count).for_each(|i| {
            if sum <= 10 {
                sum += 11
            } else {
                sum += 1
            }
        });
        sum
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_hand() {
        let hand = Hand::new();

        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn strong_hand() {
        let mut hand = Hand::new();
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn risky_hand() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Queen);
        hand.add(Card::Ace);

        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn oops() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Seven);
        hand.add(Card::Five);

        assert!(hand.is_loosing_hand());
        assert_eq!(hand.value(), 22);
    }
}