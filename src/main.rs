#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

mod editor;
mod terminal;

use editor::Editor;
use terminal::Terminal;

fn main() {
    Editor::new().start();
}
