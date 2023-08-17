use std::collections::{HashMap, HashSet, LinkedList};
#[derive(Debug, PartialEq, Hash, Eq)]
struct Name(String);
struct FileSystem{
    current_directory: Directory,
    directories: HashMap<Name, Directory>,
    files: HashSet<File>,
}
struct Directory{
    parent: Name,
    name: Name,
    dirs: HashMap<Name, Directory>,
    files: HashMap<Name, File>,
    size: u64
}
impl Directory{
    pub fn new(parent: Name, name: Name) -> Self{
        Self{ parent, name, dirs: HashMap::new(), files: HashMap::new(), size: 0}
    }
}
struct File{
    size: u64,
    name: Name,
    parent: Name,
}
impl File{
    pub fn new(name: &Name, parent: &Name, size: u64) -> Self{
        Self { size, name: *name, parent: *parent }
    }
}

#[derive(Debug, PartialEq)]
enum Command{
    ChangeDirectory(Name),
    List,
}
fn parse_command(i: &str) -> Option<Command>{
    let args = i.split_whitespace().collect::<Vec<_>>();
    match *args.get(0).unwrap(){
        "$" => {
            match *args.get(1).unwrap(){
                "cd" => Some(Command::ChangeDirectory(Name(args.get(2).unwrap().to_string()))),
                "ls" => Some(Command::List),
                _ => unimplemented!()
            }
        },
        _ => None
    }
}
fn parse_ls_line_info(i: &str, curr: &mut Directory) {
    let args = i.split_whitespace().collect::<Vec<_>>();
    match *args.get(0).unwrap(){
        "dir" => {
            let name = Name(String::from(*args.get(1).unwrap()));
            curr.dirs.insert(name, Directory::new(curr.name, name));
        },
        s => {
            match s.parse::<u64>(){
                Ok(size) => {
                    let file_name = Name(String::from(*args.get(1).unwrap()));
                    curr.files.insert(file_name, File::new(file_name, curr.name, size));
                },
                Err(e) => println!("Expect line format to be <size> <filename>, got {:?}", args),
            }
        }
    }
}
fn main() {
    for line in include_str!("input.txt").lines(){
        println!("{line}");
    }
}
#[cfg(test)]
mod test{
    use crate::{parse_command, Command, Name};

    #[test]
    fn test_parse_command_cd(){
        assert_eq!(parse_command("$ cd /"), Some(Command::ChangeDirectory(Name("/".to_string()))));
        assert_eq!(parse_command("$ cd .."), Some(Command::ChangeDirectory(Name("..".to_string()))));        
        assert_eq!(parse_command("$ cd abc"), Some(Command::ChangeDirectory(Name("abc".to_string()))));
        // assert_eq!(parse_command("$ ef ."), None);
    }
    #[test]
    fn test_parse_command_ls(){
        assert_eq!(parse_command("$ ls"), Some(Command::List));
    }
}