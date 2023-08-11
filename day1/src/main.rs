use std::fs::read_to_string;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let groups = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();
    println!("groups: {groups:?}");
    Ok(())
}
