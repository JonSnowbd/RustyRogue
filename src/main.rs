extern crate pancurses;
mod gamelogic;

fn main() {
    use gamelogic::*;
    use pancurses::*;

    // Create the map
    let mut x = map::GameMap::new();

    // Hollow out the map
    for row in 1..19{
        for val in 1..19{
            x.set_tile(val, row, -1);
        }
    }

    // Init window
    let window:Window = initscr();
    set_title("Rusty Rogue!");
    noecho();

    // Basic player position
    let mut player_position = (6i32, 6i32);

    // Mainloop
    loop{
        use pancurses::Input::*;

        // Draw map
        window.erase();
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
                Character('E') => break,
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
