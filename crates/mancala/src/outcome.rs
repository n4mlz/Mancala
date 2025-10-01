#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Outcome {
    Ongoing,
    Win(super::Player),
    Draw,
}
