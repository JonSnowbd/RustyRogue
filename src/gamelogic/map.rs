use std::collections::HashMap;

/// Basic struct that Defines the(Non-entity) world and the i32->character lookup for id->tilecharacter
///
/// All tiles are held in memory as an i32 which is representative of the ID of the tile.
/// The lookup is used to translate each ID into a character to be represented in the
/// game.
///
/// @TODO: vec!s for tiles, faster string rendering.
pub struct GameMap{
    pub lookup:HashMap<i32, char>,
    pub tiles:[[i32; 20]; 20]
}
impl GameMap {

    /// Basic constructor for a map.
    pub fn new() -> GameMap {
        GameMap{
            tiles:[[0i32; 20]; 20],
            lookup:GameMap::default_lookup_map()
        }
    }

    /// Uses the lookup to find the `char` of each tile and returns the map as a formatted
    /// block of text. Best used with Monospaced square fonts by the way.
    pub fn to_string(&self) -> String {
        // The string to be built.
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

    /// Determines if the block at `x, y` is solid or free to move through.
    pub fn is_solid(&self, x:usize, y:usize) -> bool {
        self.tiles[y][x] >= 0
    }

    /// Simple helper to 'select' a tile from the map.
    pub fn select(&self, x:usize, y:usize) -> &i32{
        &self.tiles[y][x]
    }

    /// Basic helper to change tiles during runtime.
    ///
    /// Useful for explosions or toggled gates.
    pub fn set_tile(&mut self, x:usize, y:usize, val:i32){
        self.tiles[y][x] = val;
    }

    /// Basic helper to set a default hashmap for the world.
    ///
    /// If the ID is less than 0, it will be uncollidable, above or equal to 0 will be a solid tile.
    pub fn default_lookup_map() -> HashMap<i32, char>{
        let mut tile_lookup = HashMap::new();
        tile_lookup.insert(1, '~');
        tile_lookup.insert(0, '#');
        tile_lookup.insert(-1, ',');

        tile_lookup
    }
}