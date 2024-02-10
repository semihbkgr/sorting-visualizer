use super::{
    AlgorithmContext,
    Operation::{Compare, Insert, Noop},
};

pub const NAME: &str = "merge sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    merge_sort(nums, 0, nums.len() - 1, ctx);
    ctx.next(Noop(), nums.to_vec());
}

fn merge_sort(nums: &mut [i32], low: usize, high: usize, ctx: &dyn AlgorithmContext) {
    if low < high {
        let mid = low + (high - low) / 2;

        // Recursively sort each half
        merge_sort(nums, low, mid, ctx);
        merge_sort(nums, mid + 1, high, ctx);

        // Merge the sorted halves in place
        merge(nums, low, mid, high, ctx);
    }
}

fn merge(nums: &mut [i32], low: usize, mut mid: usize, high: usize, ctx: &dyn AlgorithmContext) {
    let mut i = low;
    let mut j = mid + 1;

    while i <= mid && j <= high {
        ctx.next(Compare(i, j), nums.to_vec());
        if nums[i] <= nums[j] {
            i += 1;
        } else {
            let temp = nums[j];
            for k in (i..=mid).rev() {
                nums[k + 1] = nums[k];
            }
            nums[i] = temp;
            ctx.next(Insert(i), nums.to_vec());

            i += 1;
            j += 1;
            mid += 1;
        }
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
