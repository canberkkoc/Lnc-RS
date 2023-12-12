use filter::filter_entries;

mod gui;
mod filter;
pub fn main() {
    let entry_vector = filter::get_desktop_entries();
    filter_entries(entry_vector, "fr");
    let _ = gui::gui_run();
}
