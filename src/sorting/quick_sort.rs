use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "quick sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    quick_sort_recursive(nums, 0, nums.len() - 1, ctx);
    ctx.next(Noop(), nums.to_vec());
}

fn quick_sort_recursive(nums: &mut [i32], low: usize, high: usize, ctx: &dyn AlgorithmContext) {
    if low < high {
        let pivot_index = partition(nums, low, high, ctx);

        if pivot_index > 0 {
            quick_sort_recursive(nums, low, pivot_index - 1, ctx);
        }

        quick_sort_recursive(nums, pivot_index + 1, high, ctx);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize, ctx: &dyn AlgorithmContext) -> usize {
    let pivot = nums[high];
    let mut i = low;

    for j in low..high {
        ctx.next(Compare(j, high), nums.to_vec());
        if nums[j] <= pivot {
            if i != j {
                nums.swap(i, j);
                ctx.next(Swap(i, j), nums.to_vec());
            }
            i += 1;
        }
    }

    if i != high {
        nums.swap(i, high);
        ctx.next(Swap(i, high), nums.to_vec());
    }

    return i;
}

#[cfg(test)]
mod tests {
    use crate::sorting::has_nums;
    use crate::sorting::is_sorted;
    use crate::sorting::NoopContext;

    use super::*;

    #[test]
    fn test_sort() {
        let nums = &mut [3, 5, 2, 8, 6, 9, 0, 1, 4, 7];
        sort(nums, &NoopContext);
        assert!(is_sorted(nums));
        assert!(has_nums(nums));
    }
}
