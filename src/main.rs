extern crate pancurses;
mod gamelogic;

fn main() {
    let frontend = gamelogic::frontends::pancurses_frontend::PancursesRenderer::new();
    let mut game_app = gamelogic::app::GameApplication::new(&frontend);
    game_app.start();
}
