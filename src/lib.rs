use rand::seq::SliceRandom;

pub mod sorting;

pub fn init_vec(n: usize) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as i32);
    }
    return v;
}

pub fn shuffle(v: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_vec() {
        let v = init_vec(9);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], v);
    }

    #[test]
    fn test_shuffle() {
        let mut v = init_vec(9);
        shuffle(&mut v);
        assert_ne!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], v);
        for i in 0..9 {
            assert!(v.contains(&i));
        }
    }
}
