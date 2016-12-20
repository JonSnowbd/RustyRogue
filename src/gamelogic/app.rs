use super::map;

/// A simple representation of an Entity such as the player, a mob, an item drop etc.
pub struct Entity{
    representation: char,
    position: (i32, i32)
}

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
    fn draw_at(&self, x:i32, y:i32, string:&str);
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
            entities: Vec::new(),
            render_handle: rh
        }
    }

    pub fn start(&'a mut self){
        self.render_handle.init();
        for x in 1..19{
            for y in 1..19{
                self.map.set_tile(x, y, -1);
            }
        }
        loop{
            self.render_handle.clear();
            self.render_handle.draw_at(0,0,&self.map.to_string() as &str);

            match self.render_handle.await_input(){
                AppInput::Exit => break,
                _ => {}
            }

        }
        self.render_handle.end();
    }

}