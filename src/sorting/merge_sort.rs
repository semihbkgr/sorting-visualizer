use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "merge sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    merge_sort(nums, 0, nums.len() - 1, ctx);
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
        if nums[i] <= nums[j] {
            i += 1;
        } else {
            // Shift the elements from arr[i..=mid] to the right by one position
            let temp = nums[j];
            for k in (i..=mid).rev() {
                nums[k + 1] = nums[k];
            }
            nums[i] = temp;

            i += 1;
            j += 1;
            mid += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::is_sorted;
    use crate::sorting::NoopContext;

    use super::*;

    #[test]
    fn test_sort() {
        let nums = &mut [3, 5, 2, 8, 6, 9, 0, 1, 4, 7];
        sort(nums, &NoopContext);
        assert!(is_sorted(nums));
    }
}
