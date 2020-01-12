#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
}

impl Vec3 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
        }
    }
    #[allow(dead_code)]
    pub fn new_from(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, v: &Self) -> &mut Self {
        self.x+= v.x;
        self.y+= v.y;
        self
    }
    #[allow(dead_code)]
    pub fn sub(&mut self, v: &Self) -> &mut Self {
        self.x-= v.x;
        self.y-= v.y;
        self
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, v: f32) -> &mut Self {
        self.x*= v;
        self.y*= v;
        self
    }
    #[allow(dead_code)]
    pub fn div(&mut self, v: f32) -> &mut Self {
        self.x/= v;
        self.y/= v;
        self
    }
    #[allow(dead_code)]
    pub fn limit(&mut self, v: f32) -> &mut Self {
        let mag = self.mag();
        if mag > v {
            self.mul(v/mag);
        }
        self
    }
    #[allow(dead_code)]
    pub fn limit_min(&mut self, v: f32) -> &mut Self {
        let mag = self.mag();
        if mag < v {
            self.mul(v/mag);
        }
        self
    }
    #[allow(dead_code)]
    pub fn norm(&mut self, v: f32) -> &mut Self {
        let mag = self.mag();
        if mag > 0.0 {
            self.mul(v/mag);
        }
        self
    }
    #[allow(dead_code)]
    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    #[allow(dead_code)]
    pub fn dist(&self, v: &Self) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2)).sqrt()
    }
    #[allow(dead_code)]
    pub fn dist_mod(&self, v: &Self, w: f32, h: f32) -> f32 {
        self.dist(&self.rel(&v, w, h))
    }
    #[allow(dead_code)]
    pub fn rel(&self, v: &Self, w: f32, h: f32) -> Self {
        let x = if (self.x - v.x).abs() < w/2.0 { v.x } else { v.x + if v.x < w/2.0 { w } else { -w } };
        let y = if (self.y - v.y).abs() < h/2.0 { v.y } else { v.y + if v.y < h/2.0 { h } else { -h } };

        Self { x, y }
    }
}
