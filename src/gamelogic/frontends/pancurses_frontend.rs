use pancurses::*;
use gamelogic::app::*;

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
        set_title("Working!")
    }
    fn draw_at(&self, x:i32, y:i32, string:&str){
        self.window.mv(y, x);
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