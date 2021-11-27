#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub size: u64,
    pub used: u64,
    pub avail: u64,
}
