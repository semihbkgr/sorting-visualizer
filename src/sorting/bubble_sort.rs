use super::{
    AlgorithmContext,
    Operation::{Compare, Swap},
};

pub fn sort(nums: &mut [i32], ctx: &dyn AlgorithmContext) {
    let len = nums.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            ctx.next(Compare(nums[j], nums[j + 1]), nums.to_vec());
            if nums[j] > nums[j + 1] {
                ctx.next(Swap(nums[j], nums[j + 1]), nums.to_vec());
                nums.swap(j, j + 1);
            }
        }
    }
}
