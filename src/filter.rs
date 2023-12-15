use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct FilteredEntry {
    pub name: String,
    pub exec_path: String,
    pub icon_path: String,
}

impl Display for FilteredEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn sanitize_exec(exec: &str) -> String {
    let mut exec_path = String::new();
    for ch in exec.chars() {
        match ch {
            // Ignore all parameter
            '%' => break,
            _ => exec_path.push(ch),
        }
    }
    return exec_path;
}

pub fn get_desktop_entries() -> Vec<FilteredEntry> {
    let mut entry_vector = Vec::new();
    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                if entry.terminal() {
                    continue;
                }
                let mut tmp_obj = FilteredEntry::default();
                if let Some(val) = entry.name(None) {
                    tmp_obj.name = val.to_string()
                }
                if let Some(val) = entry.exec() {
                    tmp_obj.exec_path = sanitize_exec(val)
                }
                if let Some(val) = entry.icon() {
                    tmp_obj.icon_path = val.to_string()
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
) -> Vec<FilteredEntry> {
    let mut entry_vector: Vec<(i64, FilteredEntry)> = Vec::new();
    let matcher = SkimMatcherV2::default();

    for entry in app_entries {
        if let Some(point) = matcher.fuzzy_match(&entry.name, search_for) {
            entry_vector.push((point, entry));
        }
    }
    entry_vector.sort_by(|a, b| b.0.cmp(&a.0));
    match limit_entry_count {
        0 => entry_vector.iter().map(|x| x.1.clone()).collect(),
        val => entry_vector[0..usize::try_from(val).unwrap()]
            .into_iter()
            .map(|x| x.1.clone())
            .collect(),
    }
}
