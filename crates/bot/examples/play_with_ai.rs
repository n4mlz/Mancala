//! Play against an MCTS bot on the terminal. No asserts; uses stdin.

use std::io::{self, Write};

use bot::{RandomEvaluator, SearchConfig, mcts_search};
use mancala::{Outcome, Player, State};

fn mcts_pick(state: &State, sims: u32) -> Option<usize> {
    let eval = RandomEvaluator::default();
    let cfg = SearchConfig {
        simulations: sims,
        c_puct: 1.2,
    };
    mcts_search(state, cfg, &eval).chosen_action
}

fn main() {
    let mut s = State::new();

    // Choose sides
    println!("== Play vs AI ==");
    println!("Choose your side: A or B (default: A)");
    print!("> ");
    io::stdout().flush().ok();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let you = match buf.trim() {
        "B" | "b" => Player::B,
        _ => Player::A,
    };
    let ai = you.opponent();
    let sims_per_move = 50000;

    println!("You are {you}. AI is {ai}.");
    println!("{s}");

    while !s.is_terminal() {
        if s.current_player() == you {
            // Human turn
            let legal = s.legal_moves();
            if legal.is_empty() {
                println!("No legal moves for you. Skipping…");
            } else {
                loop {
                    print!("Your move (pit index {:?}): ", legal);
                    io::stdout().flush().ok();
                    buf.clear();
                    io::stdin().read_line(&mut buf).ok();
                    if let Ok(i) = buf.trim().parse::<usize>()
                        && let Some(ns) = s.child_after_move(i)
                    {
                        s = ns;
                        break;
                    }
                    println!("Invalid. Try again.");
                }
            }
            println!("{s}");
        } else {
            // AI turn
            let Some(a) = mcts_pick(&s, sims_per_move) else {
                println!("AI has no legal move. Skipping…");
                continue;
            };
            println!("AI ({ai}) plays pit index {a}");
            s = s.child_after_move(a).expect("AI chose legal move");
            println!("{s}");
        }
    }

    match s.outcome() {
        Outcome::Win(p) if p == you => println!(
            "You win!  score A={}, B={}",
            s.store(Player::A),
            s.store(Player::B)
        ),
        Outcome::Win(_) => println!(
            "AI wins.   score A={}, B={}",
            s.store(Player::A),
            s.store(Player::B)
        ),
        Outcome::Draw => println!(
            "Draw.      score A={}, B={}",
            s.store(Player::A),
            s.store(Player::B)
        ),
        Outcome::Ongoing => println!("Ongoing? (should not happen)"),
    }
}
