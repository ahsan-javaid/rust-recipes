use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    checkins: HashMap<i32, (String, i32)>,
    travels: HashMap<(String, String), (i32, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkins.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, end_station: String, end_time: i32) {
        if let Some((start_station, start_time)) = self.checkins.get(&id) {
            self.travels
                .entry((start_station.to_string(), end_station))
                .and_modify(|(total, trips)| {
                    *total += end_time - start_time;
                    *trips += 1;
                })
                .or_insert((end_time - start_time, 1));
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(&(total, len)) = self.travels.get(&(start_station, end_station)) {
            total as f64 / len as f64
        } else {
            0.0
        }
    }
}
fn main() {
    
}
