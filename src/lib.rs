use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct InitilizationSeed {
    length_pendulum_first: u64,
    length_pendulum_second: u64,
    mass_first: u64,
    mass_second: u64,
    tension_first: u64,
    tension_second: u64,
    radius: u64
}

impl InitilizationSeed {
    pub fn new<T: Display>(seed: T) -> Option<Self> {
        let x = format!("{}", seed);
        if x.len() < 32 {
            return None
        }
        let _k = &(x.as_bytes()[0..31]);

        let _l: u64 = 32;

        // 3*l/4 + i; i < l/4; l 

        //
        // k1 = K[i], k2 = K[L/4+i], k3 = K[L/2+i], k4 = K[3*L/4+i]
        //
        let mut k = [
            Vec::<u64>::new(),
            Vec::<u64>::new(),
            Vec::<u64>::new(),
            Vec::<u64>::new(),
        ];
        
        for i in 0.._l/4 {
            k[0].push(_k[i as usize].into());
            k[1].push(_k[(_l/4+i) as usize].into());
            k[2].push(_k[(_l/2+i) as usize].into());
            k[3].push(_k[(3*_l/4+i) as usize].into());
        }

        let mut a: Vec<InitilizationSeed> = vec!();
        
        for _ in 0..64 {
            let mut i = 0;
            let mut length_pendulum_first: u64 = 0;
            let mut length_pendulum_second: u64 = 0;
            let mut mass_first: u64 = 0;
            let mut mass_second: u64 = 0;
            let mut tension_first: u64 = 0;
            let mut tension_second: u64 = 0;
            let mut radius: u64 = 0;

            while _l/4 - i > 0 {
                length_pendulum_first = k[0][i as usize]^k[1][i as usize];
                length_pendulum_second = k[0][i as usize]^k[2][i as usize];
                mass_first = k[0][i as usize]^k[3][i as usize];
                mass_second = k[1][i as usize]^k[2][i as usize];
                tension_first = k[1][i as usize]^k[3][i as usize];
                tension_second = k[2][i as usize]^k[3][i as usize];
                radius = k[0][i as usize];
                i += 1;
            }

            a.push(Self{
                length_pendulum_first,
                length_pendulum_second,
                mass_first,
                mass_second,
                tension_first,
                tension_second,
                radius,  
            });

        }

        let mut result = Self {
            length_pendulum_first: 0,
            length_pendulum_second: 0,
            mass_first: 0,
            mass_second: 0,
            tension_first: 0,
            tension_second: 0,
            radius: 0,
        };

        for i in 0..64 {
            result.length_pendulum_first += a[i as usize].length_pendulum_first * (2 as u64).pow(63-i);
            result.length_pendulum_second += a[i as usize].length_pendulum_second * (2 as u64).pow(63-i);
            result.mass_first += a[i as usize].mass_first * (2 as u64).pow(63-i);
            result.mass_second += a[i as usize].mass_second * (2 as u64).pow(63-i);
            result.tension_first += a[i as usize].tension_first * (2 as u64).pow(63-i);
            result.tension_second += a[i as usize].tension_second * (2 as u64).pow(63-i);
            result.radius += a[i as usize].radius * (2 as u64).pow(63-i);
        }

        Some(Self {
            length_pendulum_first: result.length_pendulum_first / (2 as u64).pow(63),
            length_pendulum_second: result.length_pendulum_second / (2 as u64).pow(63),
            mass_first: result.mass_first / (2 as u64).pow(63),
            mass_second: result.mass_second / (2 as u64).pow(63),
            tension_first: result.tension_first / (2 as u64).pow(63) * 3,
            tension_second: result.tension_second / (2 as u64).pow(63) * 3,
            radius: result.radius / (2 as u64).pow(59),
        })
    }
}

///
/// Generate random seed
/// 
/// * `seed` - Need 32 length
///
/// # Examples
/// ```
/// use dprand::random_seed;
/// 
/// let seed = random_seed("example123456789qwertyuiopasdfghj");
/// match seed {
///     Some(x) => {
///         println!("{}", x);
///     },
///     None => {
///         println!("Seed is undefined");
///     }
/// }
/// ```
pub fn random_seed<T: Display>(seed: T) -> Option<u64> {
    match crate::InitilizationSeed::new(seed) {
        Some(x) => x,
        None => return None,
    };
    None
}

///
/// Generate random number
/// 
pub fn random() -> u64 {
    let x: u64;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => x = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    random_seed(x).expect("Pizdec lol")
}

#[cfg(test)]
mod tests {
    use crate::random_seed;

    #[test]
    pub fn seed_init() {
        let seed = random_seed("example123456789qwertyuiopasdfghjsd");
        assert_eq!(seed, None);
    }
}
