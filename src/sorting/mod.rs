pub mod bubble_sort;

pub trait AlgorithmContext {
    fn next(&self, operation: Operation, nums: Vec<i32>);
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Noop(),
    Compare(usize, usize),
    Swap(usize, usize),
}

struct NoopContext;

impl AlgorithmContext for NoopContext {
    fn next(&self, _: Operation, _: Vec<i32>) {}
}

#[cfg(test)]
fn is_sorted(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}
