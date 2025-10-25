use std::collections::HashMap;
use std::time::Instant;

pub struct RateTracker {
    // (name, bytes_received, bytes_sent)
    prev_network: HashMap<String, (u64, u64)>,

    // (name, bytes_read, bytes_written)
    prev_disk: HashMap<String, (u64, u64)>,
    last_update: Instant,
}

impl RateTracker {
    pub fn new() -> Self {
        Self {
            prev_network: HashMap::new(),
            prev_disk: HashMap::new(),
            last_update: Instant::now(),
        }
    }

    // return vec because iteration is easier
    pub fn update_network_rates(
        &mut self,
        current_data: &HashMap<String, (u64, u64)>,
    ) -> Vec<(String, u64, u64)> {
        let elapsed_secs = self.last_update.elapsed().as_secs_f64();
        let mut rates = Vec::new();

        // avoid division by 0
        if elapsed_secs == 0.0 {
            return rates;
        }

        for (name, &(curr_recv, curr_sent)) in current_data {
            if let Some(&(prev_recv, prev_sent)) = self.prev_network.get(name) {
                let recv_rate = (curr_recv.saturating_sub(prev_recv) as f64 / elapsed_secs) as u64;
                let sent_rate = (curr_sent.saturating_sub(prev_sent) as f64 / elapsed_secs) as u64;

                rates.push((name.clone(), recv_rate, sent_rate));
            }
        }

        self.prev_network = current_data.clone();
        self.last_update = Instant::now();

        return rates;
    }

    pub fn update_disk_rates(
        &mut self,
        current_data: &HashMap<String, (u64, u64)>,
    ) -> Vec<(String, u64, u64)> {
        let elapsed_secs = self.last_update.elapsed().as_secs_f64();
        let mut rates = Vec::new();

        if elapsed_secs == 0.0 {
            return rates;
        }

        for (name, &(curr_read, curr_write)) in current_data {
            if let Some(&(prev_read, prev_write)) = self.prev_disk.get(name) {
                let read_rate = (curr_read.saturating_sub(prev_read) as f64 / elapsed_secs) as u64;
                let write_rate = (curr_write.saturating_sub(prev_write) as f64 / elapsed_secs) as u64;

                rates.push((name.clone(), read_rate, write_rate));
            }
        }

        self.prev_disk = current_data.clone();

        return rates;
    }
}
