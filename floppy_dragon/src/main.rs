use bracket_lib::prelude::*;

mod game;
mod player;
mod obstacle;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, game::State::new())
}
