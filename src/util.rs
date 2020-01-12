#[allow(dead_code)]
pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ste = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    ste.as_secs() as f64 + ste.subsec_micros() as f64 / 1_000_000.0
}


#[allow(dead_code)]
pub struct Timer {
    times: Vec<(String, f64)>,
}

#[allow(dead_code)]
impl Timer {
    pub fn new(label: &str) -> Self {
        let mut x = Self {
            times: vec![],
        };
        x.tick(label);
        x
    }
    pub fn clear(&mut self) {
        self.times.clear();
    }
    pub fn tick(&mut self, label: &str) -> (String, f64) {
        self.times.push((label.to_string(), now()));
        self.times.get(self.times.len()-2).unwrap_or(&("start".to_string(), 0.0)).clone()
    }
    pub fn diff_or_0(&self, a: usize, b: usize) -> f64 {
        let a = self.times.get(a).unwrap_or(&("start".to_string(), 0.0)).1;
        let b = self.times.get(b).unwrap_or(&("start".to_string(), a)).1;
        a-b
    }
    pub fn show(&self) {
        for i in 0..self.times.len() {
            println!("{:>30}: {:.3}", self.times[i].0, self.diff_or_0(i, i-1));
        }
        println!("{:>30}: {:.3}", "total", self.diff_or_0(self.times.len()-1, 0));
    }
}
