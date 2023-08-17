use std::collections::HashSet;
fn find_position_marker(input: &str, size: usize) -> Option<usize>{
    
    input.as_bytes().windows(size).position(|window| window.iter().collect::<HashSet<_>>().len() == size).map(|pos| pos+size)
}
fn main() {
    let line = include_str!("input.txt");
    let count = find_position_marker(line, 14);
    println!("{}", count.unwrap());

}
#[cfg(test)]
mod tests{
    use crate::find_position_marker;

    #[test]
    fn test_sample_data_4_distinct(){
        assert_eq!(find_position_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
        assert_eq!(find_position_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(find_position_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
        assert_eq!(find_position_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
        assert_eq!(find_position_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4 ), Some(11));
    }
    #[test]
    fn test_sample_data_14_distinct(){
        assert_eq!(find_position_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(find_position_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(find_position_marker("nppdvjthqldpwncqszvftbrmjlhg",  14), Some(23));
        assert_eq!(find_position_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14), Some(29));
        assert_eq!(find_position_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some(26));
    }
}