use std::fs::File;
use std::io::{self, Read};
use color_eyre::{self};
enum Direction{
    Top,
    Left,
    Bottom, 
    Right
}
#[derive(Debug, PartialEq)]
struct Tree{
    value: usize, 
    row_idx: usize,
    col_idx: usize
}

impl Tree{
    fn is_edge(&self, grid_size: usize) -> bool {
     self.row_idx == 0 || self.col_idx == 0 || self.row_idx == grid_size-1 || self.col_idx == grid_size-1
    }
    fn is_next_to_edge(&self, grid_size: usize, direction: &Direction) -> bool{
        match direction{
            Direction::Top => self.row_idx == 1,
            Direction::Bottom => self.row_idx == grid_size - 2,
            Direction::Left => self.col_idx == 1,
            Direction::Right => self.col_idx == grid_size - 2
        }
    }
    fn is_higher_than(&self, other: &Tree) -> bool{
        self.value > other.value
    }
    fn is_lower_than(&self, other: &Tree) -> bool{
        self.value < other.value
    }
    fn is_not_higher_than(&self, other: &Tree) -> bool{
        self.value <= other.value
    }
}
#[derive(Debug)]
struct Grid{
    rows: Vec<Vec<Tree>>,
}
impl Grid{
    fn new() -> Self{
        Self{rows: Vec::new()}
    }
    fn from(input_path: &str) -> Result<Grid, io::Error>{
        let mut grid = Grid::new();
        let mut lines = String::new();
        File::open(input_path)?.read_to_string(&mut lines)?;

            for (row_idx,line) in lines.lines().enumerate(){
        let row = line.chars().enumerate().map(|(col_idx, c)| (col_idx, c.to_string().parse::<usize>())).map(|(col_idx, r)| Tree{value: r.unwrap(), row_idx,col_idx }).collect::<Vec<_>>();
        grid.rows.push(row);
    }
    Ok(grid)
    }
    fn size(&self) -> usize{
        self.rows.len()
    }
    fn get_tree_from_idxs(&self, pos: (usize, usize)) -> Option<&Tree>{
        if pos.0 >= self.size() && pos.1 >= self.size(){
            return None;
        }
        else{
            return Some(self.rows.get(pos.0).unwrap().get(pos.1).unwrap());
        }
    }
    fn check_tree_visibility(&self, tree: &Tree) -> bool{
        if tree.is_edge(self.size()){
            return true;
        }
        self.check_visibility(tree, Direction::Bottom) || self.check_visibility(tree, Direction::Left) || self.check_visibility(tree, Direction::Right) || self.check_visibility(tree, Direction::Top)
    }
    // start from: 
    // tree in-between:

    fn check_visibility(&self, tree: &Tree, direction: Direction) -> bool{
        // Position of the edge tree corresponding to the current tree in a particular direction.
        let grid_size = self.size();
        let edge_tree_position: (usize, usize) = match direction{
            Direction::Top => (0, tree.col_idx),
            Direction::Bottom => (grid_size-1, tree.col_idx),
            Direction::Left => (tree.row_idx, 0),
            Direction::Right => (tree.row_idx, grid_size -1)
        };
        let edge_tree = self.get_tree_from_idxs(edge_tree_position).unwrap();
        if tree.is_next_to_edge(grid_size, &direction) && tree.is_not_higher_than(edge_tree){
            return false;
        } else if !tree.is_next_to_edge(grid_size, &direction){
            let in_between_tree_idxs = match direction{
                Direction::Top => 0..tree.row_idx,
                Direction::Bottom => tree.row_idx+1..grid_size,
                Direction::Left => 0..tree.col_idx,
                Direction::Right => tree.col_idx+1..grid_size
            };
            for idx in in_between_tree_idxs{
                // Trees that are in the same direction with the current tree, and between that tree and the edge.
                let in_between_tree = match direction{
                    Direction::Top | Direction::Bottom => self.get_tree_from_idxs((idx, tree.col_idx)).unwrap(),
                    Direction::Left | Direction::Right => self.get_tree_from_idxs((tree.row_idx, idx)).unwrap()
                };
                if tree.is_not_higher_than(in_between_tree){
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() -> color_eyre::Result<()> {
    let mut grid = Grid::from("src/input.txt")?;
    println!("{}", grid.size());
    let mut count = 0;
    for row in grid.rows.iter(){
        let row_count = row.iter().filter(|tree| grid.check_tree_visibility(tree)).count();
        dbg!(row_count);
        count += row.iter().filter(|tree| grid.check_tree_visibility(tree)).count();
    }
    println!("{count}");
    Ok(())
}
#[cfg(test)]
mod tests{
    use crate::{Grid, Direction};
    #[test]
    fn test_left_visibility(){
        let grid = Grid::from("src/test_input.txt").unwrap();
        let top_left_5 = grid.get_tree_from_idxs((1 , 1)).unwrap();
        let top_middle_5 = grid.get_tree_from_idxs((1 , 2)).unwrap();
        let bottom_middle_5 = grid.get_tree_from_idxs((3 , 2)).unwrap();
        assert_eq!(grid.check_visibility(top_left_5, Direction::Left), true);
        assert_eq!(grid.check_visibility(bottom_middle_5, Direction::Left), true);
        assert_eq!(grid.check_visibility(top_middle_5, Direction::Left), false);
    }
    #[test]
    fn test_right_visibility(){
        let grid = Grid::from("src/test_input.txt").unwrap();
        let top_left_5 = grid.get_tree_from_idxs((1 , 1)).unwrap();
        let top_middle_5 = grid.get_tree_from_idxs((1 , 2)).unwrap();
        let left_middle_5 = grid.get_tree_from_idxs((2, 1)).unwrap();
        let right_middle_3 = grid.get_tree_from_idxs((2, 3)).unwrap();
        assert_eq!(grid.check_visibility(top_middle_5, Direction::Right), true);
        assert_eq!(grid.check_visibility(left_middle_5, Direction::Right), true);
        assert_eq!(grid.check_visibility(right_middle_3, Direction::Right), true);
        assert_eq!(grid.check_visibility(top_left_5, Direction::Right), false);
    }
    #[test]
    fn test_top_visibility(){
        let grid = Grid::from("src/test_input.txt").unwrap();
        let top_left_5 = grid.get_tree_from_idxs((1 , 1)).unwrap();
        let left_middle_5 = grid.get_tree_from_idxs((2, 1)).unwrap();
        let top_middle_5 = grid.get_tree_from_idxs((1 , 2)).unwrap();

        assert_eq!(grid.check_visibility(top_left_5, Direction::Top), true);
        assert_eq!(grid.check_visibility(top_middle_5, Direction::Top), true);
        assert_eq!(grid.check_visibility(left_middle_5, Direction::Top), false);
    }
    #[test]
    fn test_bottom_visibility(){
        let grid = Grid::from("src/test_input.txt").unwrap();
        let bottom_middle_5 = grid.get_tree_from_idxs((3 , 2)).unwrap();
        let left_middle_5 = grid.get_tree_from_idxs((2, 1)).unwrap();
        assert_eq!(grid.check_visibility(bottom_middle_5, Direction::Bottom), true);
        assert_eq!(grid.check_visibility(left_middle_5, Direction::Bottom), false);
    }
    #[test]
    fn test_no_visibility(){
        let grid = Grid::from("src/test_input.txt").unwrap();
        let center_3 = grid.get_tree_from_idxs((2 , 2)).unwrap();
        let top_right_1 = grid.get_tree_from_idxs((1 , 3)).unwrap();
        assert_eq!(grid.check_visibility(center_3, Direction::Top), false);
        assert_eq!(grid.check_visibility(center_3, Direction::Bottom), false);
        assert_eq!(grid.check_visibility(center_3, Direction::Left), false);
        assert_eq!(grid.check_visibility(center_3, Direction::Right ), false);
        assert_eq!(grid.check_visibility(top_right_1, Direction::Top), false);
        assert_eq!(grid.check_visibility(top_right_1, Direction::Bottom), false);
        assert_eq!(grid.check_visibility(top_right_1, Direction::Left), false);
        assert_eq!(grid.check_visibility(top_right_1, Direction::Right ), false);
    }
}
