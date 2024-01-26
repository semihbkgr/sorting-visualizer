use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let len = nums.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            ctx.next(Compare(j, j + 1), nums.to_vec());
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                ctx.next(Swap(j, j + 1), nums.to_vec());
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
        //descending
        let nums = &mut [3, 5, 2, 8, 6, 9, 0, 1, 8, 7];
        sort(nums, &NoopContext);
        assert!(is_sorted(nums));
    }
}
