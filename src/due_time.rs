    use std::{thread::{sleep}, time::{Duration, Instant}};
    pub struct Time {
        time_from_create: Instant,
    }
    
    impl Time {
        pub fn new() -> Self {
            
            let s = Self {
                time_from_create: Instant::now(),
            };

            sleep(Duration::new(0, 100));

            s
        }

        pub fn get_tick(&self) -> u128 {            
            Instant::now()
                .duration_since(self.time_from_create)
                .as_nanos()
        }
    }