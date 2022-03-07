use due_time::Time;

pub mod due_time;

#[allow(unused)]
#[derive(Debug)]
pub struct DoublePendulum {
    length_of_rod: (f64, f64),
    mass_of_rod: (f64, f64),
    tension_of_rod: (f64, f64),
    gravity : f64,
    seed: i64
}

// Struct Point - Author: https://github.com/srirajshukla
#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new_def() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn location(&mut self, x: f64, y: f64) -> () {
        self.x = x;
        self.y = y;
    }

    pub fn x(&self) -> f64{
        self.x
    }
    pub fn y(&self) -> f64{
        self.y
    }
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
        let gravity  = GRAVITY;

        Self {
            length_of_rod,
            mass_of_rod,
            tension_of_rod,
            gravity,
            seed,
        }
    }
}

impl Default for DoublePendulum {
    fn default() -> Self {
        Self {
            length_of_rod: (1.0, 1.0),
            mass_of_rod: (2.0, 2.0),
            tension_of_rod: (5.0,5.0),
            gravity : GRAVITY,
            seed: 0,
        }
    }
}

pub struct GenerationPendulum {
    c1: Point,
    c2: Point,
    v1: f64,
    v2: f64,
    acc1: f64,
    acc2: f64,
    double_pendulum: DoublePendulum,
}

impl GenerationPendulum {
    pub fn new(double_pendulum: DoublePendulum) -> GenerationPendulum {
        Self {
            c1: Point::new(0.0, 0.0),
            c2: Point::new(0.0, 0.0),
            v1: 0.0,
            v2: 0.0,
            acc1: 0.0,
            acc2: 0.0,
            double_pendulum: double_pendulum,
        }
    }
    fn calc_pos(&mut self) {
        self.c1.x = self.double_pendulum.length_of_rod.0 * self.double_pendulum.tension_of_rod.0.sin();
        self.c1.y = -1.0*self.double_pendulum.length_of_rod.0 * self.double_pendulum.tension_of_rod.0.cos();
        self.c2.x = self.c1.x + self.double_pendulum.length_of_rod.1 * self.double_pendulum.tension_of_rod.1.sin();
        self.c2.y = self.c1.y - self.double_pendulum.length_of_rod.1 * self.double_pendulum.tension_of_rod.1.cos();
    }

    fn new_acc(&mut self) {
        let p1 = -self.double_pendulum.gravity*(2.0*self.double_pendulum.mass_of_rod.0 + self.double_pendulum.mass_of_rod.1)*self.double_pendulum.tension_of_rod.0.sin();
        let p2 = -self.double_pendulum.mass_of_rod.1*self.double_pendulum.gravity*(self.double_pendulum.tension_of_rod.0 - 2.0*self.double_pendulum.tension_of_rod.1).sin();
        let p3 = -2.0*(self.double_pendulum.tension_of_rod.0 - self.double_pendulum.tension_of_rod.1).sin()*self.double_pendulum.mass_of_rod.1;
        let p4 = self.v2*self.v2*self.double_pendulum.length_of_rod.1 + self.v1*self.v1*(self.double_pendulum.tension_of_rod.0 - self.double_pendulum.tension_of_rod.1).cos();
        let p5 = 2.0*self.double_pendulum.mass_of_rod.0 + self.double_pendulum.mass_of_rod.1 - self.double_pendulum.mass_of_rod.1*(2.0*self.double_pendulum.tension_of_rod.0 - 2.0*self.double_pendulum.tension_of_rod.1).cos();
        
        self.acc1 = (p1 + p2 + p3*p4) / (self.double_pendulum.length_of_rod.0*p5);

        let p1 = 2.0*(self.double_pendulum.tension_of_rod.0 - self.double_pendulum.tension_of_rod.1).sin();
        let p2 = self.v1*self.v1*self.double_pendulum.length_of_rod.0 *(self.double_pendulum.mass_of_rod.0 + self.double_pendulum.mass_of_rod.1);
        let p3 = self.double_pendulum.gravity*(self.double_pendulum.mass_of_rod.0 + self.double_pendulum.mass_of_rod.1) * self.double_pendulum.tension_of_rod.0.cos();
        let p4 = self.v2*self.v2*self.double_pendulum.length_of_rod.1*self.double_pendulum.mass_of_rod.1*(self.double_pendulum.tension_of_rod.0 - self.double_pendulum.tension_of_rod.1).cos();

        self.acc2 = p1*(p2 + p3 + p4)/(self.double_pendulum.length_of_rod.1*p5);
    }

    fn new_angle(&mut self) {
        self.double_pendulum.tension_of_rod.0 += self.v1;
        self.double_pendulum.tension_of_rod.1 += self.v2;
    }

    fn new_vel(&mut self) {
        self.v1 += 0.2*self.acc1;
        self.v2 += 0.2*self.acc2;
    }

    fn new_pos(&mut self) {
        self.new_vel();
        self.new_acc();
        self.new_angle();
        self.calc_pos();
    }

    pub fn generate(&mut self) -> (&Point, &Point) {
        self.calc_pos();

        for _ in 0..1000 {
            self.new_pos();
        }

        (&self.c1, &self.c2)
    }

    pub fn create_seed() -> i64 {
        let timer = Time::new();
        for _ in 0..100 {
            
        }

        (timer.get_tick() as i64).pow(2) * 10
    }
}

#[cfg(test)]
mod tests {
    use crate::{DoublePendulum, GenerationPendulum};

    #[test]
    pub fn create_dobule_pendulum() {
        let double_pendulum = DoublePendulum::new(7788787945454);
        
        println!("{:?}", double_pendulum);
    }

    #[test]
    pub fn generation() {
        let seed = GenerationPendulum::create_seed();
        println!("{}", seed);
        let mut generator = GenerationPendulum::new(DoublePendulum::new(seed));
        println!("{:?}", generator.generate());
    }

    #[test]
    pub fn create_seed() {
        for _ in 1..10 {
            println!("{}", GenerationPendulum::create_seed());
        }
    }
}
