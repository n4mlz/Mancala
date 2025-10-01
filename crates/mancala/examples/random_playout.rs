use mancala::{Outcome, State};
use rand::seq::IndexedRandom;

fn main() {
    let mut rng = rand::rng();

    let mut s = State::new();
    println!("Initial:\n{}\n", s);

    for step in 0..512 {
        match s.outcome() {
            Outcome::Ongoing => {
                let who = s.current_player();
                let moves = s.legal_moves();
                if moves.is_empty() {
                    break;
                }
                let &mv = moves.choose(&mut rng).unwrap();
                let child = s.child_after_move(mv).unwrap();

                println!("Step {step}: {} plays pit {mv}\n{}\n", who, child);

                s = child;
            }
            fin => {
                println!("Final outcome: {fin:?}\n{}", s);
                break;
            }
        }
    }
}
