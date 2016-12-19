use std::collections::HashMap;

// Basic Structure of a game map with its own tile set
// and Tile list.
pub struct GameMap{
    pub lookup:HashMap<i32, char>,
    pub tiles:[[i32; 20]; 20]
}

impl GameMap {

    // Converts the tiles ids to a string based on the lookup hashmap
    pub fn to_string(&self) -> String {
        let mut result_string = String::new();

        for row in 0..self.tiles.len(){
            for val in self.tiles[row].iter(){
                match self.lookup.get(val){
                    Some(t) => result_string.push(*t),
                    None => result_string.push(' ')
                }
            }
            result_string.push('\n');
        }

        result_string
    }

    // Basic helper to change tiles.
    pub fn set_tile(&mut self, x:usize, y:usize, val:i32){
        self.tiles[y][x] = val;
    }
}