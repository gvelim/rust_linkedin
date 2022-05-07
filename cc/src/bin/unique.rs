use std::collections::HashSet;
use std::hash::Hash;

fn unique<T>(v: Vec<T>) -> Vec<T>
    where T: Ord + Copy + Hash {

    let mut map = HashSet::new();
    let mut output : Vec<T> = Vec::default();

    v.into_iter()
        .for_each(|item| {
            map.insert(item)
                .then(|| output.push(item));
        });
    output
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::unique;

    #[test]
    fn test_no_dup() {
        let v = vec![1,5,6,3,2];
        assert_eq!(unique(v), vec![1,5,6,3,2]);
    }
    #[test]
    fn test_dup() {
        let v = vec![1,5,1,5,2,5,3,2,3,2,2,3];
        assert_eq!(unique(v), vec![1,5,2,3]);
    }
}
