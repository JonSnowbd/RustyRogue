extern crate pancurses;
use pancurses::*;
use std::collections::HashMap;
mod rust_like_logic;

fn main() {

    // Create the tile lookup for TileID => Char
    let mut tile_lookup = HashMap::new();
    tile_lookup.insert(0, '#');
    tile_lookup.insert(1, '@');
    
    // Create the map
    let mut x = rust_like_logic::map::GameMap{
        tiles:[[0i32; 20]; 20],
        lookup:tile_lookup
    };

    // Hollow out the map
    for row in 1..19{
        for val in 1..19{
            x.set_tile(val, row, -1);
        }
    }

    // Init window
    let window:Window = initscr();

    // Basic player position
    let mut player_position = (10i32, 10i32);

    // Mainloop
    loop{
        use pancurses::Input::*;

        // Draw map
        window.clear();
        window.printw(&x.to_string() as &str);
        window.refresh();

        // Draw player
        window.mv(player_position.1, player_position.0);
        window.printw("@");
        window.mv(0,21);
        window.printw("WASD to move, E to exit.");
        
        // Query Input
        match window.getch(){
            Some(key) =>
            match key{
                Character('e') => break,
                Character('w') => player_position.1 -= 1,
                Character('a') => player_position.0 -= 1,
                Character('s') => player_position.1 += 1,
                Character('d') => player_position.0 += 1,
                _ => {}
            },
            None => break
        }
    }

    
    endwin();

}
