use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, Foundations, GameState, ACE, JACK, KING, QUEEN};

#[test]
fn test_solved() {
    let expected = GameState {
        cascades: [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ],
        foundations: Foundations([
            vec![
                Card { suit: Club, rank: ACE },
                Card { suit: Club, rank: 2 },
                Card { suit: Club, rank: 3 },
                Card { suit: Club, rank: 4 },
                Card { suit: Club, rank: 5 },
                Card { suit: Club, rank: 6 },
                Card { suit: Club, rank: 7 },
                Card { suit: Club, rank: 8 },
                Card { suit: Club, rank: 9 },
                Card { suit: Club, rank: 10 },
                Card { suit: Club, rank: JACK },
                Card { suit: Club, rank: QUEEN },
                Card { suit: Club, rank: KING },
            ],
            vec![
                Card { suit: Spade, rank: ACE },
                Card { suit: Spade, rank: 2 },
                Card { suit: Spade, rank: 3 },
                Card { suit: Spade, rank: 4 },
                Card { suit: Spade, rank: 5 },
                Card { suit: Spade, rank: 6 },
                Card { suit: Spade, rank: 7 },
                Card { suit: Spade, rank: 8 },
                Card { suit: Spade, rank: 9 },
                Card { suit: Spade, rank: 10 },
                Card { suit: Spade, rank: JACK },
                Card { suit: Spade, rank: QUEEN },
                Card { suit: Spade, rank: KING },
            ],
            vec![
                Card { suit: Heart, rank: ACE },
                Card { suit: Heart, rank: 2 },
                Card { suit: Heart, rank: 3 },
                Card { suit: Heart, rank: 4 },
                Card { suit: Heart, rank: 5 },
                Card { suit: Heart, rank: 6 },
                Card { suit: Heart, rank: 7 },
                Card { suit: Heart, rank: 8 },
                Card { suit: Heart, rank: 9 },
                Card { suit: Heart, rank: 10 },
                Card { suit: Heart, rank: JACK },
                Card { suit: Heart, rank: QUEEN },
                Card { suit: Heart, rank: KING },
            ],
            vec![
                Card { suit: Diamond, rank: ACE },
                Card { suit: Diamond, rank: 2 },
                Card { suit: Diamond, rank: 3 },
                Card { suit: Diamond, rank: 4 },
                Card { suit: Diamond, rank: 5 },
                Card { suit: Diamond, rank: 6 },
                Card { suit: Diamond, rank: 7 },
                Card { suit: Diamond, rank: 8 },
                Card { suit: Diamond, rank: 9 },
                Card { suit: Diamond, rank: 10 },
                Card { suit: Diamond, rank: JACK },
                Card { suit: Diamond, rank: QUEEN },
                Card { suit: Diamond, rank: KING },
            ],
        ]),
        freecells: [None, None, None, None],
    };

    let actual = GameState::from_file("test-inputs/solved.txt").unwrap();

    assert_eq!(actual, expected);
}
