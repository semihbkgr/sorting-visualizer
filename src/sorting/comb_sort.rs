use super::{
    AlgorithmContext,
    Operation::{Compare, Noop, Swap},
};

pub const NAME: &str = "comb sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let n = nums.len();
    let mut gap = n;
    let shrink_factor = 1.3;
    let mut swapped = true;

    while gap > 1 || swapped {
        gap = (gap as f64 / shrink_factor).floor() as usize;
        if gap < 1 {
            gap = 1;
        }

        swapped = false;

        for i in 0..n - gap {
            let j = i + gap;
            ctx.next(Compare(i, j), nums.to_vec());
            if nums[i] > nums[j] {
                nums.swap(i, j);
                ctx.next(Swap(i, j), nums.to_vec());
                swapped = true;
            }
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
