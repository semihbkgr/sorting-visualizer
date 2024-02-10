use std::fmt::Display;

pub mod bubble_sort;
pub mod comb_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;

pub trait AlgorithmContext {
    fn next(&self, operation: Operation, nums: Vec<i32>);
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Noop(),
    Compare(usize, usize),
    Swap(usize, usize),
    Insert(usize),
}

impl Operation {
    pub fn adjusted(&self) -> Self {
        return match *self {
            Self::Compare(a, b) => {
                if a > b {
                    Self::Compare(b, a)
                } else {
                    Self::Compare(a, b)
                }
            }
            Self::Swap(a, b) => {
                if a > b {
                    Self::Swap(b, a)
                } else {
                    Self::Swap(a, b)
                }
            }
            Self::Insert(i) => return Self::Insert(i),
            Self::Noop() => return Self::Noop(),
        };
    }
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
            Self::Insert(i) => {
                write!(f, "insert: {}", i)
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
        shell_sort::NAME,
        heap_sort::NAME,
        quick_sort::NAME,
        comb_sort::NAME,
    ];
}

pub fn get_algorithm_func<'a>(s: &str) -> impl FnOnce(&mut [i32], &dyn AlgorithmContext) {
    match s {
        bubble_sort::NAME => bubble_sort::sort,
        selection_sort::NAME => selection_sort::sort,
        insertion_sort::NAME => insertion_sort::sort,
        merge_sort::NAME => merge_sort::sort,
        shell_sort::NAME => shell_sort::sort,
        heap_sort::NAME => heap_sort::sort,
        quick_sort::NAME => quick_sort::sort,
        comb_sort::NAME => comb_sort::sort,
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
fn has_nums(nums: &[i32]) -> bool {
    for e in 0..=9 {
        if !nums.contains(&e) {
            return false;
        }
    }
    return true;
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
