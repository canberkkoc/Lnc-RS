use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct FilteredEntry {
    name: String,
    exec_path: String,
    icon_path: String,
}

pub fn get_desktop_entries() -> Vec<FilteredEntry> {
    let mut entry_vector = Vec::new();
    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                let mut tmp_obj = FilteredEntry::default();
                match entry.name(None) {
                    Some(val) => tmp_obj.name = val.to_string(),
                    _ => (),
                }
                match entry.exec() {
                    Some(val) => tmp_obj.exec_path = val.to_string(),
                    _ => (),
                }
                match entry.icon() {
                    Some(val) => tmp_obj.icon_path = val.to_string(),
                    _ => (),
                }
                entry_vector.push(tmp_obj)
            }
        }
    }
    entry_vector
}

pub fn filter_entries(
    app_entries: Vec<FilteredEntry>,
    search_for: &str,
    limit_entry_count: i32,
) -> Vec<(i64, FilteredEntry)> {
    let mut entry_vector: Vec<(i64, FilteredEntry)> = Vec::new();
    let matcher = SkimMatcherV2::default();

    for entry in app_entries {
        if let Some(point) = matcher.fuzzy_match(&entry.name, search_for) {
            entry_vector.push((point, entry));
        }
    }
    entry_vector.sort_by(|a, b| b.0.cmp(&a.0));
    match limit_entry_count {
        0 => entry_vector,
        val => entry_vector[0..usize::try_from(val).unwrap()].to_vec(),
    }
}
