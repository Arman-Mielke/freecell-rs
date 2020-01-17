use lazy_static::lazy_static;
use regex::Regex;

use crate::card::CARD_PATTERN;
use crate::{Card, CardCollection};

/// A stack of arbitrary cards.
///
/// The end of the `Vec` is the top of the stack.
///
/// Can be parsed with [`parse_cascade`](fn.parse_cascade.html).
///
/// # Rules
///
/// Adding cards:
/// A card can be put on a cascade iff its rank is 1 lower than that of the top card of the cascade
/// and it has a different colour than the top card of the cascade.
///
/// Removing cards:
/// Only the top card of the cascade can be removed.
///
/// # Examples
///
/// ```
/// # use freecell::Suit::{Club, Heart, Spade};
/// # use freecell::{parse_cascade, Card, CardCollection, ACE};
/// let cascade = parse_cascade("9S AC 7H").unwrap();
/// assert_eq!(
///     cascade,
///     vec![
///         Card { suit: Spade, rank: 9 },
///         Card { suit: Club, rank: ACE },
///         Card { suit: Heart, rank: 7 },
///     ]
/// );
///
/// // The 6 of Spades fits on top of the 7 of Hearts,
/// // since it is of a different colour and one rank lower.
/// assert_eq!(
///     cascade.add_card(Card { suit: Spade, rank: 6 }),
///     Ok(vec![
///         Card { suit: Spade, rank: 9 },
///         Card { suit: Club, rank: ACE },
///         Card { suit: Heart, rank: 7 },
///         Card { suit: Spade, rank: 6 },
///     ])
/// );
///
/// // Only the top card of the cascade can be removed.
/// assert_eq!(
///     cascade.pop_card(),
///     vec![(
///         vec![
///             Card { suit: Spade, rank: 9 },
///             Card { suit: Club, rank: ACE },
///         ],
///         Card { suit: Heart, rank: 7 }
///     )]
/// );
/// ```
// TODO [v1] make this a tuple struct and implement Display, Debug and FromStr for it
// TODO [v1] the formats for Display and Debug must be consistent with FromStr (test this!)
pub type Cascade = Vec<Card>;

fn fits_on_top_of(lower_card: Card, higher_card: Card) -> bool {
    lower_card.suit.colour() != higher_card.suit.colour() &&
    lower_card.rank + 1 == higher_card.rank
}

impl CardCollection for Cascade {
    fn add_card(&self, card: Card) -> Result<Cascade, ()> {
        match self.last() {
            // the cascade contains at least one card
            Some(&top_card) => {
                if fits_on_top_of(card, top_card) {
                    // the new card can be put onto this cascade
                    let mut clone = (*self).clone();
                    clone.push(card);
                    Ok(clone)
                } else {
                    // the new card cannot be put onto this cascade
                    Err(())
                }
            }

            // the cascade is empty => the card can be put here, creating a cascade with one card
            None => Ok(vec![card]),
        }
    }

    fn pop_card(&self) -> Vec<(Cascade, Card)> {
        let mut clone = (*self).clone();
        match clone.pop() {
            Some(card) => vec![(clone, card)],
            None => Vec::with_capacity(0),
        }
    }
}

/// Converts a string to a [`Cascade`](type.Cascade.html).
///
/// The input string should consist of any number of cards, where the cards follow the format
/// described in [`Card`](struct.Card.html)'s `FromStr` implementation.
/// Cards can optionally be separated by spaces.
///
/// # Examples
///
/// ```
/// # use freecell::Suit::{Club, Heart, Spade};
/// # use freecell::{parse_cascade, Card, ACE};
/// assert_eq!(parse_cascade(""), Ok(Vec::new()));
///
/// assert_eq!(
///     parse_cascade("9S AC 7H"),
///     Ok(vec![
///         Card { suit: Spade, rank: 9 },
///         Card { suit: Club, rank: ACE },
///         Card { suit: Heart, rank: 7 },
///     ])
/// );
/// ```
pub fn parse_cascade<S: Into<String>>(string: S) -> Result<Cascade, String> {
    lazy_static! {
        static ref CASCADE_RE: Regex = Regex::new(format!(r"(?i)^\s*({}\s*)*$", CARD_PATTERN).as_str()).unwrap();
        static ref CARD_RE: Regex = Regex::new(format!(r"(?i){}", CARD_PATTERN).as_str()).unwrap();
    }

    let string = &string.into();
    if !CASCADE_RE.is_match(string) {
        return Err(format!("Could not parse cascade: \"{}\"", string))
    }

    Ok(CARD_RE.find_iter(string).map(|re_match| re_match.as_str().parse().unwrap()).collect())
}

/// A collection of 8 Cascades.
pub type Cascades = [Cascade; 8];
