//! History sorting utilities

use crate::models::HistoryEntry;

/// Sort history entries chronologically (oldest first)
pub fn sort_history_chronologically(history: &mut [HistoryEntry]) {
    history.sort_by(|a, b| {
        let a_time = a.created.as_deref().unwrap_or("");
        let b_time = b.created.as_deref().unwrap_or("");
        a_time.cmp(b_time)
    });
}

/// Sort history entries and return a new vector
pub fn sorted_history_chronologically(history: &[HistoryEntry]) -> Vec<HistoryEntry> {
    let mut sorted = history.to_vec();
    sort_history_chronologically(&mut sorted);
    sorted
}

/// Check if history is sorted chronologically
pub fn is_history_sorted_chronologically(history: &[HistoryEntry]) -> bool {
    history.windows(2).all(|w| {
        let a_time = w[0].created.as_deref().unwrap_or("");
        let b_time = w[1].created.as_deref().unwrap_or("");
        a_time <= b_time
    })
}
