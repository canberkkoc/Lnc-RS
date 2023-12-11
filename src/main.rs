use filter::filter_entries;

mod gui;
mod filter;
pub fn main() {
    let vectorius = filter::get_desktop_entries();
    filter_entries(vectorius, "frfox");
    let _ = gui::gui_run();
}
