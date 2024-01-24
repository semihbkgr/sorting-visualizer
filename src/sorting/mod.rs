pub mod bubble_sort;

pub trait AlgorithmContext {
    fn next(&self, operation: Operation, nums: Vec<i32>);
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Noop(),
    Compare(i32, i32),
    Swap(i32, i32),
}
