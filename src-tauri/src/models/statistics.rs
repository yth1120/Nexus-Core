use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeRange {
    #[serde(rename = "7d")]
    SevenDays,
    #[serde(rename = "30d")]
    ThirtyDays,
    #[serde(rename = "1y")]
    OneYear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficDataPoint {
    pub timestamp: i64,
    pub upload: i64,
    pub download: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsData {
    pub today_traffic: i64,
    pub month_traffic: i64,
    pub month_quota: i64,
    pub max_speed: i64,
    pub max_speed_date: String,
    pub history: Vec<TrafficDataPoint>,
    pub daily_averages: Vec<f64>,
}

impl Default for StatisticsData {
    fn default() -> Self {
        Self {
            today_traffic: 0,
            month_traffic: 0,
            month_quota: 322_122_547_200, // 300 GB
            max_speed: 0,
            max_speed_date: String::new(),
            history: Vec::new(),
            daily_averages: Vec::new(),
        }
    }
}
