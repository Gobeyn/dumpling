pub mod color;
pub mod file;
pub mod key;
pub mod ui;

use file::parser;

fn main() {
    let mut filepath = dirs::cache_dir().expect("Error");
    filepath.push("dumpling/1.toml");
    let paper: parser::Paper = parser::parse_paper_toml(&mut filepath).unwrap();
    println!("{:?}", paper)
}

fn main_v2() {
    ui::window::create_window();
}
