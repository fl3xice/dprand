
pub mod due_time;

#[allow(unused)]
#[derive(Debug)]
pub struct DoublePendulum {
    length_of_rod: (f64, f64),
    mass_of_rod: (f64, f64),
    tension_of_rod: (f64, f64),
    radius: f64,
    seed: i64
}

pub const GRAVITY : f64 = 9.8;

impl DoublePendulum {
    /// 
    /// #### Initialization Parameters
    /// 
    /// ### Example
    /// ```
    /// DoublePendulum::new(514642);
    /// // or
    /// DoublePendulum::new(0);
    /// ```
    /// 
    pub fn new(seed: i64) -> DoublePendulum {
        let mut s = seed.to_be_bytes();

        s.reverse();

        if !(s.iter().filter(|x| **x != 0 ).count() >= 6) {
            return Self::default()
        }

        let length_of_rod = (s[0] as f64 / 10.0, s[1] as f64 / 10.0);
        let mass_of_rod = (s[2] as f64 / 10.0, s[3] as f64 / 10.0);
        let tension_of_rod = (s[4] as f64 / 10.0, s[5] as f64 / 10.0);
        let radius = GRAVITY;

        Self {
            length_of_rod,
            mass_of_rod,
            tension_of_rod,
            radius,
            seed,
        }
    }
}

struct GenerationPendulum {
    point: (f64, f64)

}

impl GenerationPendulum {
    pub fn new_pos(&self) {
        
    }

    pub fn generate(dp: DoublePendulum) {
        let timer = crate::due_time::Time::new();
        let t0 = dp.radius * dp.seed as f64;
        let (i, j, k, S) = (0f64,0f64,0f64,0f64);
        let (v1,v2) = (0f64, 0f64);
        let F = u64::MAX.to_string().len() as u64;
        let h = 0.002f64;
        
        
        for _ in 0..F {
            
        }
        
    }
}



impl Default for DoublePendulum {
    fn default() -> Self {
        Self {
            length_of_rod: (1.0, 1.0),
            mass_of_rod: (2.0, 2.0),
            tension_of_rod: (5.0,5.0),
            radius: GRAVITY,
            seed: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublePendulum;
    use crate::generate;

    #[test]
    pub fn create_dobule_pendulum() {
        let double_pendulum = DoublePendulum::new(7788787945454);
        
        println!("{:?}", double_pendulum);
    }

    #[test]
    pub fn generation() {
        let double_pendulum = DoublePendulum::new(7788787945454);
        generate(double_pendulum);
    }
}
