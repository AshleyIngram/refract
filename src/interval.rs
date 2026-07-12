pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_contains_correct() {
        let interval = Interval::new(1.0, 2.0);
        assert!(interval.contains(1.0));
        assert!(interval.contains(1.5));
        assert!(interval.contains(2.0));
        assert!(!interval.contains(2.5));
    }

    #[test]
    fn interval_surrounds_correct() {
        let interval = Interval::new(1.0, 2.0);
        assert!(!interval.surrounds(1.0));
        assert!(interval.surrounds(1.5));
        assert!(!interval.surrounds(2.0));
        assert!(!interval.surrounds(2.5));
    }

    #[test]
    fn interval_clamp_correct() {
        let interval = Interval::new(1.0, 2.0);

        assert_eq!(interval.clamp(0.5), 1.0);
        assert_eq!(interval.clamp(1.5), 1.5);
        assert_eq!(interval.clamp(2.5), 2.0);
    }
}
