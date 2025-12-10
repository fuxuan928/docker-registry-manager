//! Sorting utilities

/// Sort strings alphabetically (case-insensitive)
pub fn sort_alphabetically(items: &mut [String]) {
    items.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}

/// Sort strings alphabetically and return a new vector
pub fn sorted_alphabetically(items: &[String]) -> Vec<String> {
    let mut sorted = items.to_vec();
    sort_alphabetically(&mut sorted);
    sorted
}

/// Check if a slice is sorted alphabetically
pub fn is_sorted_alphabetically(items: &[String]) -> bool {
    items.windows(2).all(|w| w[0].to_lowercase() <= w[1].to_lowercase())
}
