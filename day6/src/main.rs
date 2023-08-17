use std::collections::HashSet;

use itertools::Itertools;
fn find_position_marker(input: &str) -> Option<usize>{
    
    input.as_bytes().windows(4).position(|window| window.iter().collect::<HashSet<_>>().len() == 4).map(|pos| pos+4)
}
fn main() {
    let line = include_str!("input.txt");
    let count = find_position_marker(line);
    println!("{}", count.unwrap());
}
#[cfg(test)]
mod tests{
    use crate::find_position_marker;

    #[test]
    fn test_sample_data(){
        assert_eq!(find_position_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(find_position_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(find_position_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(find_position_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(find_position_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }
}