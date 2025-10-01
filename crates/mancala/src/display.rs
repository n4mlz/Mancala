use crate::{PITS_PER_SIDE, Player, State};
use std::fmt::{self, Display, Formatter};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m"; // Player::A
const MAGENTA: &str = "\x1b[35m"; // Player::B

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Player::A => write!(f, "{CYAN}A{RESET}"),
            Player::B => write!(f, "{MAGENTA}B{RESET}"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let a = Player::A;
        let b = Player::B;

        let nums_b_plain = fmt_row_rev_plain(self.pits(b));
        let nums_a_plain = fmt_row_plain(self.pits(a));
        let idx_b_plain = fmt_idx_row_rev_plain();
        let idx_a_plain = fmt_idx_row_plain();

        let line1_plain = format!("|    B: [{}]     |", nums_b_plain);
        let line1i_plain = format!("|    B: [{}]     |", idx_b_plain);
        let line3_plain = format!("|    A: [{}]     |", nums_a_plain);
        let line3i_plain = format!("|    A: [{}]     |", idx_a_plain);

        let target_width = line1_plain.len();
        debug_assert_eq!(line3_plain.len(), target_width);
        debug_assert_eq!(line1i_plain.len(), target_width);
        debug_assert_eq!(line3i_plain.len(), target_width);

        let store_b_plain = format!("[B:{:>2}]", self.store(b));
        let store_a_plain = format!("[A:{:>2}]", self.store(a));

        let inside_width = target_width - 2;
        let left_pad = 2usize;
        let right_pad = 2usize;
        let core_min_plain = left_pad + store_b_plain.len() + store_a_plain.len() + right_pad;
        let gap = inside_width.saturating_sub(core_min_plain);

        let label_a_col = if self.current_player() == a {
            format!("{BOLD}{CYAN}A{RESET}")
        } else {
            format!("{CYAN}A{RESET}")
        };
        let label_b_col = if self.current_player() == b {
            format!("{BOLD}{MAGENTA}B{RESET}")
        } else {
            format!("{MAGENTA}B{RESET}")
        };

        let nums_b_col = fmt_row_rev_col(self.pits(b), MAGENTA);
        let nums_a_col = fmt_row_col(self.pits(a), CYAN);
        let idx_b_col = fmt_idx_row_rev_col();
        let idx_a_col = fmt_idx_row_col();

        let line1_col = format!("|    {label_b_col}: [{}]     |", nums_b_col);
        let line1i_col = format!("|    {label_b_col}: [{}]     |", idx_b_col);
        let line3_col = format!("|    {label_a_col}: [{}]     |", nums_a_col);
        let line3i_col = format!("|    {label_a_col}: [{}]     |", idx_a_col);

        let store_b_col = format!("{MAGENTA}[B:{:>2}]{RESET}", self.store(b));
        let store_a_col = format!("{CYAN}[A:{:>2}]{RESET}", self.store(a));

        let line2_col = format!(
            "|{}{}{}{}{}|",
            " ".repeat(left_pad),
            store_b_col,
            " ".repeat(gap),
            store_a_col,
            " ".repeat(right_pad),
        );

        writeln!(f, "{line1_col}")?;
        writeln!(f, "{line1i_col}")?;
        writeln!(f, "{line2_col}")?;
        writeln!(f, "{line3_col}")?;
        writeln!(f, "{line3i_col}")
    }
}

fn fmt_row_plain(pits: &[u8; PITS_PER_SIDE]) -> String {
    let mut s = String::new();
    for (i, v) in pits.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{:>2}", v));
    }
    s
}

fn fmt_row_rev_plain(pits: &[u8; PITS_PER_SIDE]) -> String {
    let mut s = String::new();
    for (k, i) in (0..PITS_PER_SIDE).rev().enumerate() {
        if k > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{:>2}", pits[i]));
    }
    s
}

fn fmt_row_col(pits: &[u8; PITS_PER_SIDE], color: &str) -> String {
    let mut s = String::new();
    for (i, v) in pits.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{color}{:>2}{RESET}", v));
    }
    s
}

fn fmt_row_rev_col(pits: &[u8; PITS_PER_SIDE], color: &str) -> String {
    let mut s = String::new();
    for (k, i) in (0..PITS_PER_SIDE).rev().enumerate() {
        if k > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{color}{:>2}{RESET}", pits[i]));
    }
    s
}

fn fmt_idx_row_plain() -> String {
    let mut s = String::new();
    for i in 0..PITS_PER_SIDE {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{:>2}", i));
    }
    s
}
fn fmt_idx_row_col() -> String {
    let mut s = String::new();
    for i in 0..PITS_PER_SIDE {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{DIM}{:>2}{RESET}", i));
    }
    s
}

fn fmt_idx_row_rev_plain() -> String {
    let mut s = String::new();
    for (k, i) in (0..PITS_PER_SIDE).rev().enumerate() {
        if k > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{:>2}", i));
    }
    s
}
fn fmt_idx_row_rev_col() -> String {
    let mut s = String::new();
    for (k, i) in (0..PITS_PER_SIDE).rev().enumerate() {
        if k > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{DIM}{:>2}{RESET}", i));
    }
    s
}
