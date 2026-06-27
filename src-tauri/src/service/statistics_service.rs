use rand::Rng;

use crate::core::AppState;
use crate::models::{StatisticsData, TimeRange};

/// Get statistics data for the given time range.
/// For mock purposes, always regenerates based on time range from the stored seed data.
pub fn get_statistics(state: &AppState, time_range: &TimeRange) -> StatisticsData {
    let mut stats = state.statistics_data.read().clone();

    // Adjust history length based on time range
    match time_range {
        TimeRange::SevenDays => {
            let cutoff = stats.history.len().saturating_sub(7 * 24);
            stats.history = stats.history[cutoff..].to_vec();
            stats.daily_averages =
                stats.daily_averages[stats.daily_averages.len().saturating_sub(7)..].to_vec();
        }
        TimeRange::ThirtyDays => {
            // Full data already covers ~30 days
        }
        TimeRange::OneYear => {
            // Extend with more mock data for 1y view
            let existing = stats.history.len();
            let needed = 365 * 24;
            if existing < needed {
                let mut rng = rand::thread_rng();
                let last_ts = stats.history.last().map(|p| p.timestamp).unwrap_or(0);
                let new_points: Vec<_> = (existing..needed)
                    .map(|i| {
                        let ts = last_ts - (needed - i) as i64 * 3600 * 1000;
                        let hour = ((ts / 1000 / 3600) % 24) as f64;
                        let factor = ((hour - 6.0) * std::f64::consts::PI / 12.0).sin();
                        let factor = (factor + 1.0) / 2.0;
                        let dl = (5.0 + factor * 15.0 + rng.gen::<f64>() * 5.0) * 1024.0 * 1024.0;
                        let ul = (1.0 + factor * 4.0 + rng.gen::<f64>() * 2.0) * 1024.0 * 1024.0;
                        crate::models::TrafficDataPoint {
                            timestamp: ts,
                            upload: ul as i64,
                            download: dl as i64,
                        }
                    })
                    .collect();
                let mut full = new_points;
                full.extend(stats.history.clone());
                stats.history = full;
            }
        }
    }

    stats
}
