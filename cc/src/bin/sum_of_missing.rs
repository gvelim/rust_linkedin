fn sum_of_missing(arr: Vec<Option<i32>>) -> i32 {
    arr.iter()
        .fold(0,| sum, num| sum + num.unwrap_or(0))
}

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array() {
        let v = vec![Some(1), None, Some(4)];
        assert_eq!(
            sum_of_missing(v),
            5i32
        );
    }
    #[test]
    fn test_empty() {
        let v = vec![None,None,None];
        assert_eq!(
            sum_of_missing(v),
            0i32
        );
    }
}