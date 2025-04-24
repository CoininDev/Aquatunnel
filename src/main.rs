mod comps;
mod entitites;
mod input;
mod physics;
mod game2;
mod sys;
mod window;

fn main() {
    miniquad::start(miniquad::conf::Conf::default(), || Box::new(game2::Game::default()));
}
