use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn filter_entries(app_entries: Vec<&str>, search_for: &str) {
    let matcher = SkimMatcherV2::default();
    for i in app_entries {
        if let Some(point) = matcher.fuzzy_match(&i, search_for) {
            println!("{:?} {:?}", point, i);
        }
    }
}
