use lazy_static::lazy_static;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use crate::card::CARD_PATTERN;
use crate::{Card, CardCollection};

/// A stack of arbitrary cards.
///
/// The end of the `Vec` is the top of the stack.
///
/// Cascades can be parsed from `&str`s.
/// See the description for `FromStr` below for details.
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
/// # use freecell::{Card, CardCollection, Cascade, ACE};
/// let cascade: Cascade = "9S AC 7H".parse().unwrap();
/// assert_eq!(
///     cascade,
///     Cascade(vec![
///         Card { suit: Spade, rank: 9 },
///         Card { suit: Club, rank: ACE },
///         Card { suit: Heart, rank: 7 },
///     ])
/// );
///
/// // The 6 of Spades fits on top of the 7 of Hearts,
/// // since it is of a different colour and one rank lower.
/// assert_eq!(
///     cascade.add_card(Card { suit: Spade, rank: 6 }),
///     Ok(Cascade(vec![
///         Card { suit: Spade, rank: 9 },
///         Card { suit: Club, rank: ACE },
///         Card { suit: Heart, rank: 7 },
///         Card { suit: Spade, rank: 6 },
///     ]))
/// );
///
/// // Only the top card of the cascade can be removed.
/// assert_eq!(
///     cascade.pop_card(),
///     vec![(
///         Cascade(vec![
///             Card { suit: Spade, rank: 9 },
///             Card { suit: Club, rank: ACE },
///         ]),
///         Card { suit: Heart, rank: 7 }
///     )]
/// );
/// ```
// TODO [v1] implement Display and Debug for this
// TODO [v1] the formats for Display and Debug must be consistent with FromStr (test this!)
// TODO [low priority] make this iterable? search for ".0.iter()" and ".0.last()" for places where this can be used
// TODO                implement Deref or Index for this?
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cascade(pub Vec<Card>);

impl Cascade {
    /// Creates an empty cascade
    ///
    /// # Example
    ///
    /// ```
    /// # use freecell::Cascade;
    /// assert_eq!(Cascade::new(), Cascade(Vec::new()))
    /// ```
    pub fn new() -> Cascade {
        Cascade::default()
    }
}

fn fits_on_top_of(lower_card: Card, higher_card: Card) -> bool {
    lower_card.suit.colour() != higher_card.suit.colour() &&
    lower_card.rank + 1 == higher_card.rank
}

impl CardCollection for Cascade {
    fn add_card(&self, card: Card) -> Result<Cascade, ()> {
        match self.0.last() {
            // the cascade contains at least one card
            Some(&top_card) => {
                if fits_on_top_of(card, top_card) {
                    // the new card can be put onto this cascade
                    let mut clone = (*self).clone();
                    clone.0.push(card);
                    Ok(clone)
                } else {
                    // the new card cannot be put onto this cascade
                    Err(())
                }
            }

            // the cascade is empty => the card can be put here, creating a cascade with one card
            None => Ok(Cascade(vec![card])),
        }
    }

    fn pop_card(&self) -> Vec<(Cascade, Card)> {
        let mut clone = (*self).clone();
        match clone.0.pop() {
            Some(card) => vec![(clone, card)],
            None => Vec::with_capacity(0),
        }
    }
}

// TODO test
impl Display for Cascade {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let cards: Vec<String> = self.0.iter()
            .map(|card| card.to_string())
            .collect();
        writeln!(f, "Cascade: {}", cards.join(", "))
    }
}

impl FromStr for Cascade {
    type Err = String;

    /// Converts a `&str` to a `Cascade`.
    ///
    /// The input should consist of any number of cards, where the cards follow the format described
    /// in [`Card`](struct.Card.html)'s `FromStr` implementation.
    /// Cards can optionally be separated by spaces, commas or both.
    ///
    /// # Example
    ///
    /// ```
    /// # use freecell::Suit::{Club, Heart, Spade};
    /// # use freecell::{Card, Cascade, ACE};
    /// assert_eq!(
    ///     "9S AC 7H".parse(),
    ///     Ok(Cascade(vec![
    ///         Card { suit: Spade, rank: 9 },
    ///         Card { suit: Club, rank: ACE },
    ///         Card { suit: Heart, rank: 7 },
    ///     ]))
    /// );
    /// ```
    fn from_str(string: &str) -> Result<Cascade, Self::Err> {
        lazy_static! {
            static ref CASCADE_RE: Regex = Regex::new(format!(r"(?i)^\s*({}\s*)*$", CARD_PATTERN).as_str()).unwrap();
            static ref CARD_RE: Regex = Regex::new(format!(r"(?i){}", CARD_PATTERN).as_str()).unwrap();
        }

        if !CASCADE_RE.is_match(string) {
            return Err(format!("Could not parse cascade: \"{}\"", string))
        }

        Ok(Cascade(
            CARD_RE.find_iter(string)
                .map(|re_match| re_match.as_str().parse().unwrap())
                .collect()
        ))
    }
}

/// A collection of 8 Cascades.
pub type Cascades = [Cascade; 8];
