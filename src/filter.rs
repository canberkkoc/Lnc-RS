use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::fs;

use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter, PathSource};


pub fn get_desktop_entries() -> Vec<String>{
    let mut vectore = Vec::new();
    for path in Iter::new(default_paths()) {
        let path_src = PathSource::guess_from(&path);
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                if let Some(entriname) = entry.name(None){
                    vectore.push(entriname.to_string());
                };
            }
        }
    }
    vectore
}



pub fn filter_entries(app_entries: Vec<String>, search_for: &str) {
    let matcher = SkimMatcherV2::default();
    for i in app_entries {
        if let Some(point) = matcher.fuzzy_match(&i, search_for) {
            println!("{:?} {:?}", point, i);
        }
    }
}



