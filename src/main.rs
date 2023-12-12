mod gui;
mod filter;

use filter::{filter_entries, get_desktop_entries};
use gui::gui_run;

pub fn main() {
    filter_entries(get_desktop_entries(), "fr");
    match gui_run(){
        Ok(()) => (),
        error => println!("{:?}", error)
    }
}
