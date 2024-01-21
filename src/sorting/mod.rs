pub trait Algorithm {
    fn next(&mut self, operation: Operation);
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Compare(usize, usize),
    Swap(usize, usize),
}
