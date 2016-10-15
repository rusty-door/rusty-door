mod labyrinth;
mod menu;
mod screen;
mod state;

fn main() {
    let mut pr = state::ProgramState::new();
    pr.new_game();
    if let Some(f) = pr.game {
        println!("{}", f.field);
    }
}

