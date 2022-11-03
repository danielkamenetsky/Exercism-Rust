// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const EARTH_SECONDS: f64 = 31_55_7_600_f64;

#[derive(Debug)]
pub struct Duration (f64);

//u64 will be converted to the f64 in the duration struct
impl From<u64> for Duration {
    // Self relates to the Duration struct
    fn from(s: u64) -> Self {
        //Using brackets because struct uses a tuple
        Self(s as f64)
    }
}

// convert a duration to the number of years on this planet for that duration
pub trait Planet {
    const FACTOR: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::FACTOR  // this is how you access a tuple index
    }
    
    
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury { const FACTOR: f64= EARTH_SECONDS * 0.2408467 ;}
impl Planet for Venus { const FACTOR: f64= EARTH_SECONDS* 0.61519726; }
impl Planet for Earth {
    const FACTOR: f64 = EARTH_SECONDS * 1.0;
}
impl Planet for Mars { const FACTOR: f64= EARTH_SECONDS*1.8808158; }
impl Planet for Jupiter {const FACTOR: f64= EARTH_SECONDS* 11.862615;}
impl Planet for Saturn {const FACTOR: f64= EARTH_SECONDS* 29.447498;}
impl Planet for Uranus {const FACTOR: f64= EARTH_SECONDS*84.016846;}
impl Planet for Neptune {const FACTOR: f64= EARTH_SECONDS* 164.79132;}
