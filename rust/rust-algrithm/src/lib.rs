
mod dp;
mod graph;
mod tree;
mod greedy;
mod search;
mod stack;
mod math;
mod queue;
mod bits;
mod hash;
mod backtracking;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}