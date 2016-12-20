extern crate pancurses;
use pancurses::{Window};
mod gamelogic;


struct PancursesRenderer{
    window:Window
}
impl PancursesRenderer{
    pub fn new(w:Window) -> PancursesRenderer {
        PancursesRenderer{
            window:w
        }
    }
}

impl gamelogic::app::RenderHandle for PancursesRenderer {
    fn init(&self){
        pancurses::set_title("Working!")
    }
    fn draw_at(&self, x:i32, y:i32, string:&str){
        self.window.mv(x, y);
        self.window.printw(string);
    }
    fn clear(&self){
        self.window.erase();
    }
    fn end(&self){
        pancurses::endwin();
    }
    fn await_input(&self) -> gamelogic::app::AppInput {
        use gamelogic::app::{AppInput};
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

fn main() {
    let window:Window = pancurses::initscr();
    let pcr = PancursesRenderer::new(window);

    let mut game_app = gamelogic::app::GameApplication::new(&pcr);
    game_app.start();
}
