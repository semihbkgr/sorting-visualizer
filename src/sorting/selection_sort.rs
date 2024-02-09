use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "selection sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let len = nums.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            ctx.next(Compare(smallest, right), nums.to_vec());
            if nums[right] < nums[smallest] {
                smallest = right;
            }
        }
        if smallest != left {
            nums.swap(smallest, left);
            ctx.next(Swap(left, smallest), nums.to_vec());
        }
    }
    ctx.next(Noop(), nums.to_vec());
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
