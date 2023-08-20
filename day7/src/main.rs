use camino::{Utf8PathBuf, Utf8Path};
use nom::{IResult, Finish};
use nom::combinator::all_consuming;
use nom::sequence::{preceded, separated_pair};
use nom::{
    bytes::complete::{take_while1,tag},
    combinator::{map},
    branch::alt

};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}
#[derive(Debug)]
struct Ls;
fn parse_ls(i: &str) -> IResult<&str, Ls>{
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}
#[derive(Debug)]
enum Command{
    Ls,
    Cd(Utf8PathBuf)
}
impl From<Ls> for Command{
    fn from(value: Ls) -> Self {
        Command::Ls
    }
}
impl From<Cd> for Command{
    fn from(value: Cd) -> Self {
        Command::Cd(value.0)
    }
}
fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}
#[derive(Debug)]
enum Entry{
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf)
}
fn parse_entry(i: &str) -> IResult<&str, Entry>{
    let file = map(
        separated_pair(nom::character::complete::u64, tag(" ") ,parse_path), |(size, path)| Entry::File(size, path), 
    );
    let dir = map(
        preceded(tag("dir "), parse_path),
        Entry::Dir
    );
    alt((file, dir))(i)
}
#[derive(Debug)]
enum Line{
    Entry(Entry),
    Command(Command)
}
fn parse_line(i: &str) -> IResult<&str, Line>{
    alt(((map(parse_command, Line::Command)), map(parse_entry, Line::Entry)))(i)
}
fn main() {
    let lines = include_str!("input.txt").lines().map(|l| {
        all_consuming(parse_line)(l).finish().unwrap().1
    }).collect::<Vec<_>>();
    for line in lines{
        println!("{line:?}");
    }
}
#[cfg(test)]
mod test{
    
}