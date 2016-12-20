extern crate pancurses;
mod gamelogic;

fn main() {
    use gamelogic::*;
    use pancurses::*;

    // Create the map
    let mut themap = map::GameMap::new(20,20);

    for y in 1..19{
        for x in 1..19{
            themap.set_tile(x, y, -1);
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
        window.printw(&themap.to_string() as &str);
        window.refresh();

        // Draw player
        window.mv(player_position.1, player_position.0);
        window.printw("@");
        window.mv(0,21);
        window.printw("wasd to move, E to exit.");

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
            None => {}
        }
    }
    endwin();
}
