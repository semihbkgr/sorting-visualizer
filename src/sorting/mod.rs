use std::fmt::Display;

pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;

pub trait AlgorithmContext {
    fn next(&self, operation: Operation, nums: Vec<i32>);
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Noop(),
    Compare(usize, usize),
    Swap(usize, usize),
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Noop() => std::fmt::Result::Ok(()),
            Self::Compare(a, b) => {
                write!(f, "compare: {} {}", a, b)
            }
            Self::Swap(a, b) => {
                write!(f, "swap: {} {}", a, b)
            }
        }
    }
}

pub fn get_algorithms() -> Vec<&'static str> {
    return vec![
        bubble_sort::NAME,
        selection_sort::NAME,
        insertion_sort::NAME,
        merge_sort::NAME,
    ];
}

pub fn get_algorithm_func<'a>(s: &str) -> impl FnOnce(&mut [i32], &dyn AlgorithmContext) {
    match s {
        bubble_sort::NAME => bubble_sort::sort,
        selection_sort::NAME => selection_sort::sort,
        insertion_sort::NAME => insertion_sort::sort,
        merge_sort::NAME => merge_sort::sort,
        _ => panic!("algorithm not found"),
    }
}

struct NoopContext;

impl AlgorithmContext for NoopContext {
    fn next(&self, _: Operation, _: Vec<i32>) {}
}

#[cfg(test)]
fn is_sorted(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_algorithm_func() {
        _ = get_algorithm_func(bubble_sort::NAME);
    }

    #[test]
    #[should_panic(expected = "algorithm not found")]
    fn test_get_algorithm_func_not_found() {
        _ = get_algorithm_func("algorithm");
    }
}

//todo: call Noop outside of the algorithm fn
