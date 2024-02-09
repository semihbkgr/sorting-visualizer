use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "insertion sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    for i in 1..nums.len() + 1 {
        for j in (1..i).rev() {
            ctx.next(Compare(j - 1, j), nums.to_vec());
            if nums[j] < nums[j - 1] {
                nums.swap(j - 1, j);
                ctx.next(Swap(j - 1, j), nums.to_vec());
            } else {
                break;
            }
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
