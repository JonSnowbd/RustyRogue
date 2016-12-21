use gamelogic::map;

/// A simple representation of an Entity such as the player, a mob, an item drop etc.
pub struct Entity{
    position: (i32, i32),
    color:i16,
    representation: char,
}

/// An enum that represents input from the frontend into usable
pub enum AppInput{
    PlayerMovement{x:i32, y:i32},
    Exit,
    Nothing
}

/// The trait to be implemented on the frontend that will draw the game.
///
/// These is all the methods that will need to be used to draw the game.
pub trait RenderHandle{
    fn init(&self);
    fn clear(&self);
    fn draw_at(&self, x:i32, y:i32, string:&str, color:i16);
    fn end(&self);
    fn await_input(&self) -> AppInput;
}

/// The core of the game that handles entity interactions and drawing.
pub struct GameApplication<'a> {
    map: map::GameMap,
    entities: Vec<Entity>,
    render_handle: &'a (RenderHandle + 'a)
}

impl<'a> GameApplication<'a>{

    /// Basic constructor.
    pub fn new(rh: &'a RenderHandle) -> GameApplication<'a> {
        GameApplication{
            map: map::GameMap::new(20,20),
            entities: Vec::with_capacity(30),
            render_handle: rh
        }
    }

    /// Starts the logic/render loop and inits the front end.
    ///
    /// This is where any kind of game state belongs, could probably do with a better name
    /// to make this clearer.
    pub fn start(&'a mut self){
        self.render_handle.init();

        // Basically test code to simulate a player entity.
        let mut player = (3i32, 3i32);
        for x in 1..19{
            for y in 1..19{
                self.map.set_tile(x, y, -1);
            }
        }

        loop{
            self.render_handle.clear();
            self.render_handle.draw_at(0,0,&self.map.to_string() as &str, 1i16);


            self.render_handle.draw_at(player.0, player.1, "@", 2i16);

            self.render_handle.draw_at(23, 1, "wasd to move, E to exit(capital e)", 1i16);


            match self.render_handle.await_input(){
                AppInput::Exit => break,
                AppInput::PlayerMovement{x, y} => {
                    player.0 += x;
                    player.1 += y;
                },
                _ => {}
            }

        }
        self.render_handle.end();
    }

}