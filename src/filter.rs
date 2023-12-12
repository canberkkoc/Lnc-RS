use std::fs;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};


pub fn get_desktop_entries() -> Vec<String>{
    let mut entry_vector = Vec::new();
    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                match entry.name(None){
                    Some(val) => entry_vector.push(val.to_string()),
                    _ => ()
                }
            }
        }
    }
    entry_vector
}



pub fn filter_entries(app_entries: Vec<String>, search_for: &str) {
    let matcher = SkimMatcherV2::default();
    for i in app_entries {
        if let Some(point) = matcher.fuzzy_match(&i, search_for) {
            println!("{:?} {:?}", point, i);
        }
    }
}



