use pancurses::*;
use gamelogic::app::*;

pub const PLAYERCOLOR:i16 = 1;
pub const FLOORCOLOR:i16 = 2;

pub struct PancursesRenderer{
    window:Window
}
impl PancursesRenderer{
    pub fn new() -> PancursesRenderer {
        PancursesRenderer{
            window:initscr()
        }
    }
}

impl RenderHandle for PancursesRenderer {
    fn init(&self){
        set_title("Rusty Rogue");
        start_color();
        noecho();

        init_pair(1, 3, 0);
        init_pair(2, 2, 1);
    }
    fn draw_at(&self, x:i32, y:i32, string:&str, _color:i16){
        self.window.mv(y, x);
        self.window.color_set(_color);
        self.window.printw(string);
    }
    fn clear(&self){
        self.window.erase();
    }
    fn end(&self){
        endwin();
    }
    fn await_input(&self) -> AppInput {
        use pancurses::Input::*;
        match self.window.getch(){
            Some(key) =>
            match key{
                Character('E') => AppInput::Exit,
                Character('w') => AppInput::PlayerMovement{x:0,y:-1},
                Character('a') => AppInput::PlayerMovement{x:-1,y:0},
                Character('s') => AppInput::PlayerMovement{x:0,y:1},
                Character('d') => AppInput::PlayerMovement{x:1,y:0},
                _ => AppInput::Nothing
            },
            None => AppInput::Nothing
        }
    }
}