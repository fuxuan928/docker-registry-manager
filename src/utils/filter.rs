//! String filtering utilities

/// Filter a list of strings by a search term (case-insensitive)
pub fn filter_strings<'a>(items: &'a [String], search: &str) -> Vec<&'a String> {
    if search.is_empty() {
        return items.iter().collect();
    }
    
    let search_lower = search.to_lowercase();
    items
        .iter()
        .filter(|item| item.to_lowercase().contains(&search_lower))
        .collect()
}

/// Filter and return owned strings
pub fn filter_strings_owned(items: &[String], search: &str) -> Vec<String> {
    filter_strings(items, search)
        .into_iter()
        .cloned()
        .collect()
}
