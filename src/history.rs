use std::collections::VecDeque;

// shape for tracked data
#[derive(Debug, Clone)]
pub struct History {
    pub cpu_usage: f32,
    pub mem_percent_usage: f64,
    pub mem_used_mb: u64,
}

pub struct HistoryTracker {
    data: VecDeque<History>,
    max_size: usize,
}

// funcs on HistoryTracker
impl HistoryTracker {
    // create new instance
    pub fn new(max_size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    // mutable to be able to update instance of self
    pub fn add(&mut self, data: History) {
        // if data is full, pop oldest entry
        if self.data.len() >= self.max_size {
            self.data.pop_front();
        }
        self.data.push_back(data);
    }

    // average cpu usage over runtime
    pub fn cpu_avg(&self) -> Option<f32> {
        if self.data.is_empty() {
            // no average without data
            return None;
        }
        let sum: f32 = self.data.iter().map(|x| x.cpu_usage).sum();
        Some(sum / self.data.len() as f32)
    }

    // max cpu usage over runtime
    pub fn cpu_max(&self) -> Option<f32> {
        self.data
            .iter()
            .map(|x| x.cpu_usage)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    // average memory usage over runtime
    pub fn mem_avg(&self) -> Option<f64> {
        if self.data.is_empty() {
            // no average without data
            return None;
        }
        let sum: f64 = self.data.iter().map(|x| x.mem_percent_usage).sum();
        Some(sum / self.data.len() as f64)
    }

    // max memory usage over runtime
    pub fn mem_max(&self) -> Option<f64> {
        self.data
            .iter()
            .map(|x| x.mem_percent_usage)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    // shows if cpu usage is increasing or decreasing
    pub fn cpu_trend(&self) -> String {
        if self.data.len() < 2 {
            return "".to_string();
        }

        let curr = self.data.back().unwrap().cpu_usage;
        let prev = self.data[self.data.len() - 2].cpu_usage;

        if curr > prev + 1.0 {
            "↗".to_string()
        } else if curr < prev - 1.0 {
            "↘".to_string()
        } else {
            "→".to_string()
        }
    }

    // shows if memory usage is increasing or decreasing
    pub fn mem_trend(&self) -> String {
        if self.data.len() < 2 {
            return "".to_string();
        }

        let curr = self.data.back().unwrap().mem_percent_usage;
        let prev = self.data[self.data.len() - 2].mem_percent_usage;

        if curr > prev + 1.0 {
            "↗".to_string()
        } else if curr < prev - 1.0 {
            "↘".to_string()
        } else {
            "→".to_string()
        }
    }

    // shows other modules if we have data
    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }
}
