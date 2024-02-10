use super::{
    AlgorithmContext,
    Operation::{Compare, Insert, Noop},
};

pub const NAME: &str = "insertion sort";

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 {
            ctx.next(Compare(j - 1, i), nums.to_vec());
            if nums[j - 1] > nums[i] {
                j -= 1;
            } else {
                break;
            }
        }

        if i != j {
            let temp = nums[i];
            for k in (j..i).rev() {
                nums[k + 1] = nums[k];
            }
            nums[j] = temp;
            ctx.next(Insert(j), nums.to_vec());
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
