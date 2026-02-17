pub trait Parity {
    fn is_even(self) -> bool;
    fn is_odd(self) -> bool;
}

impl Parity for usize {
    fn is_even(self) -> bool {
        self % 2 == 0
    }
    fn is_odd(self) -> bool {
        self % 2 != 0
    }
}
