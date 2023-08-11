fn main() {
    let mut result = 0;
    let rucksacks = include_str!("input.txt").lines().map(|line| line.split_at(line.len() / 2)).collect::<Vec<_>>();
    for &r in rucksacks.iter() {
        for c in r.0.chars() {
            if r.1.contains(c) {
                if c.is_lowercase() {
                    result += c as i32 - 96;
                } else {
                    result += c as i32 - 38;
                }
                break;
            }
        }
    }
    println!("{result}");
}
