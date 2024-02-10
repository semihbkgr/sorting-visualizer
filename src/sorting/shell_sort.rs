use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "shell sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let n = nums.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap {
                ctx.next(Compare(j - gap, j), nums.to_vec());
                if nums[j - gap] > nums[j] {
                    nums.swap(j - gap, j);
                    ctx.next(Swap(j - gap, j), nums.to_vec());
                    j -= gap;
                } else {
                    break;
                }
            }
        }
        gap /= 2;
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
