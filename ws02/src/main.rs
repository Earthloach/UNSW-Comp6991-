#![allow(dead_code)]
mod tests;

use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CSVRecord {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeOfDay {
    Morning,
    Midday,
    Evening,
    Midnight,
    Total,
}

/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: You can test your `Solution` methods here manually, or call `cargo test` to execute unit tests.
    let solution = new_solution()?;

    Ok(())
}

pub struct Solution {
    // TODO: You can put whatever state you require for each query here.
    records: Vec<CSVRecord>,
    station_locations: HashMap<String, Location>,
}

pub fn new_solution() -> Result<Solution, Box<dyn Error>> {
    // TODO: Initialise the common state you will require here.

    let path = Path::new("trains.csv");

    let records: Vec<CSVRecord> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    let mut stations = HashMap::new();
    for record in records.iter() {
        stations
            .entry(record.station.clone())
            .or_insert(Location::new(record.latitude, record.longitude));
    }

    Ok(Solution {
        records,
        station_locations: stations,
    })
}

/// What is the north-most station?
pub fn find_north_most_station(solution: &Solution) -> Option<String> {
    solution
        .station_locations
        .iter()
        .max_by(|(_, loc1), (_, loc2)| loc1.latitude().total_cmp(&loc2.latitude()))
        .map(|(name, _)| name.clone())
}

/// What is the south-most station?
pub fn find_south_most_station(solution: &Solution) -> Option<String> {
    solution
        .station_locations
        .iter()
        // Find the minimum latitude
        .min_by(|(_, loc1), (_, loc2)| loc1.latitude().total_cmp(&loc2.latitude()))
        .map(|(name, _)| name.clone())
}
/// What is the east-most station?
pub fn find_east_most_station(solution: &Solution) -> Option<String> {
    solution
        .station_locations
        .iter()
        // Find the maximum longitude
        .max_by(|(_, loc1), (_, loc2)| loc1.longitude().total_cmp(&loc2.longitude()))
        .map(|(name, _)| name.clone())
}

/// What is the west-most station?
pub fn find_west_most_station(solution: &Solution) -> Option<String> {
    solution
        .station_locations
        .iter()
        // Find the minimum longitude
        .min_by(|(_, loc1), (_, loc2)| loc1.longitude().total_cmp(&loc2.longitude()))
        .map(|(name, _)| name.clone())
}

/// Return the names of the most and least used (total entries + exits) stations on the NSW network at each time of day, in total over all of the years.
pub fn most_least_used_stations(
    solution: &Solution,
    time_of_day: TimeOfDay,
) -> Option<(String, String)> {
    let mut station_usage = HashMap::new();

    solution.records.iter().for_each(|record| {
        let usage = match time_of_day {
            TimeOfDay::Morning => {
                record.entries_morning.unwrap_or(0) + record.exits_morning.unwrap_or(0)
            }
            TimeOfDay::Midday => {
                record.entries_midday.unwrap_or(0) + record.exits_midday.unwrap_or(0)
            }
            TimeOfDay::Evening => {
                record.entries_evening.unwrap_or(0) + record.exits_evening.unwrap_or(0)
            }
            TimeOfDay::Midnight => {
                record.entries_midnight.unwrap_or(0) + record.exits_midnight.unwrap_or(0)
            }
            TimeOfDay::Total => record.entries_total.unwrap_or(0) + record.exits_total.unwrap_or(0),
        };
        *station_usage.entry(record.station.clone()).or_insert(0) += usage;
    });

    let most_used = station_usage.iter().max_by_key(|&(_, usage)| *usage);
    let least_used = station_usage.iter().min_by_key(|&(_, usage)| *usage);

    match (most_used, least_used) {
        (Some((most_name, _)), Some((least_name, _))) => {
            Some((least_name.clone(), most_name.clone()))
        }
        _ => None,
    }
    // solution
    //     .station_locations
    //     .iter()
    //     .max_by()
}

// TODO: if you think the Vec return type is inefficient/unsuitable, ask your tutor about more flexible alternatives (hint: iterators).
/// Allow a user to search for a station, and show it's busiest times of day.
pub fn search_station_busiest_times_of_day(
    solution: &Solution,
    station_name: &str,
) -> Option<Vec<(TimeOfDay, i32)>> {
    let mut usage_by_time: HashMap<TimeOfDay, i32> = HashMap::new();

    // Filter records for the specified station and aggregate usage by time of day
    for record in solution
        .records
        .iter()
        .filter(|r| r.station == station_name)
    {
        *usage_by_time.entry(TimeOfDay::Morning).or_insert(0) +=
            record.entries_morning.unwrap_or(0) + record.exits_morning.unwrap_or(0);
        *usage_by_time.entry(TimeOfDay::Midday).or_insert(0) +=
            record.entries_midday.unwrap_or(0) + record.exits_midday.unwrap_or(0);
        *usage_by_time.entry(TimeOfDay::Evening).or_insert(0) +=
            record.entries_evening.unwrap_or(0) + record.exits_evening.unwrap_or(0);
        *usage_by_time.entry(TimeOfDay::Midnight).or_insert(0) +=
            record.entries_midnight.unwrap_or(0) + record.exits_midnight.unwrap_or(0);
        // Total is not included as we compare specific time slots
    }

    if usage_by_time.is_empty() {
        // Station not found or has no data
        return None;
    }

    // Convert the map to a vector
    let mut result: Vec<(TimeOfDay, i32)> = usage_by_time.into_iter().collect();

    // Sort the results by usage in descending order
    result.sort_by(|a, b| b.1.cmp(&a.1));

    Some(result)
}

/// Allow a user to search for a station, if it exists, and show it's busiest year.
pub fn search_station_busiest_year(solution: &Solution, station_name: &str) -> Option<String> {
    let mut usage_by_year: HashMap<String, i32> = HashMap::new();

    // Filter records for the specified station and aggregate total usage by year
    for record in solution
        .records
        .iter()
        .filter(|r| r.station == station_name)
    {
        // Extract the year part from the time_period string
        // Split by whitespace and take the last part.
        // Use "?." for safe navigation in case split returns empty or only one part.
        // Convert the resulting &str to String for the HashMap key.
        if let Some(year) = record.time_period.split_whitespace().last() {
            let total_usage = record.entries_total.unwrap_or(0) + record.exits_total.unwrap_or(0);
            // Use the extracted year (as a String) as the key
            *usage_by_year.entry(year.to_string()).or_insert(0) += total_usage;
        }
        // Optionally handle cases where year extraction might fail, though unlikely with expected format.
    }

    // If no data was aggregated (e.g., station not found or no valid years), return None early.
    if usage_by_year.is_empty() {
        return None;
    }

    // Find the year (String key) with the maximum total usage
    usage_by_year
        .into_iter()
        // Use max_by_key on the owned (String, i32) tuple from into_iter
        .max_by_key(|(_, usage)| *usage) // Compare by usage value
        .map(|(year, _)| year) // Return the year String
}

/// Which station had its yearly utilisation (total entries + exits) increase the most from 2016 (inclusive) to 2020 (inclusive)?
pub fn find_largest_yearly_utilisation_increase(solution: &Solution) -> Option<String> {
    todo!()
}

/// Which station had the biggest percentage change in utilisation (total entries + exits) from 2019 to 2020?
pub fn find_biggest_percentage_change(solution: &Solution) -> Option<String> {
    todo!()
}

/// Find the names of the two closest from each other.
pub fn find_two_closest_stations(solution: &Solution) -> Option<(String, String)> {
    todo!()
}

/// Find the names of the two furthest away from each other.
pub fn find_two_furthest_stations(solution: &Solution) -> Option<(String, String)> {
    todo!()
}
