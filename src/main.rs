mod gui;
mod filter;

use filter::{filter_entries, get_desktop_entries};
use gui::gui_run;

pub fn main() {
    println!{"{:?}",filter_entries(get_desktop_entries(), "go", 2)};
    match gui_run(){
        Ok(()) => (),
        error => println!("{:?}", error)
    }
}
