use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "heap sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let n = nums.len();
    for i in (0..n / 2).rev() {
        heapify(nums, n, i, ctx);
    }
    for i in (1..n).rev() {
        nums.swap(0, i);
        ctx.next(Swap(0, i), nums.to_vec());
        heapify(nums, i, 0, ctx);
    }
    ctx.next(Noop(), nums.to_vec());
}

fn heapify(nums: &mut [i32], n: usize, i: usize, ctx: &dyn AlgorithmContext) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n {
        ctx.next(Compare(left, largest), nums.to_vec());
        if nums[left] > nums[largest] {
            largest = left;
        }
    }

    if right < n {
        ctx.next(Compare(right, largest), nums.to_vec());
        if nums[right] > nums[largest] {
            largest = right;
        }
    }

    if largest != i {
        nums.swap(i, largest);
        ctx.next(Swap(i, largest), nums.to_vec());
        heapify(nums, n, largest, ctx);
    }
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
