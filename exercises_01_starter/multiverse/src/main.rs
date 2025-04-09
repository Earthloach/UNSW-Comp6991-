use std::u32;

// hmm this doesn't look right!!
struct UniverseDetails {
    universe_name: String,
    universe_winner: String,
    universe_population: u32,
}

fn get_universe_details(universe_id: u32) -> Option<UniverseDetails> {
    // does this even compile??
    // struct UniverseDetails; // Removed to avoid shadowing the original struct
    if universe_id % 3 == 0 && universe_id % 5 == 0 {
        Some(UniverseDetails {
            universe_name: "Stardew Valley".to_string(),
            universe_winner: "Jojo Corp".to_string(),
            universe_population: 1, 
        })
    } else if universe_id % 3 == 0 {
        Some(UniverseDetails {
            universe_name: "Star Wars".to_string(),
            universe_winner: "The Rebellion".to_string(),
            universe_population: u32::MAX, 
        })
    } else if universe_id % 5 == 0 {
        Some(UniverseDetails {
            universe_name: "Miraculous".to_string(),
            universe_winner: "Hawk Moth".to_string(),
            universe_population: 22,
        })
    } else {
        None
    }
}


// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of 
// ("{}", variable)
fn main() {
    for id in 1..=15 {
        if let Some(universe_details) = get_universe_details(id) {
            println!("Universe with id {} is called {}, won by {} and has a population of {}", 
                id, 
                universe_details.universe_name, 
                universe_details.universe_winner, 
                universe_details.universe_population
            );
        } else {
            println!("Universe with id {} is unknown", id);
        }
    }
}
