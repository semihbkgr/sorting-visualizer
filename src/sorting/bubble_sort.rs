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
