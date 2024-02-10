use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "bubble sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let len = nums.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - i - 1 {
            ctx.next(Compare(j, j + 1), nums.to_vec());
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                ctx.next(Swap(j, j + 1), nums.to_vec());
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    ctx.next(Noop(), nums.to_vec());
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
