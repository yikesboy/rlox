#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    const ALL: [Self; 11] = [
        Self::None,
        Self::Assignment,
        Self::Or,
        Self::And,
        Self::Equality,
        Self::Comparison,
        Self::Term,
        Self::Factor,
        Self::Unary,
        Self::Call,
        Self::Primary,
    ];

    pub fn next_n_higher(self, n: usize) -> Self {
        let index = self as usize;
        let next = index.saturating_add(n).min(Self::ALL.len() - 1);

        Self::ALL[next]
    }
}
