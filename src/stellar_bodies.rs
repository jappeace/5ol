// this file describes stellar objects
pub type Au = f64;

use time::Duration;
use geometry::*;

pub struct StellarBody{
    pub name:&'static str,
    pub orbitTime:Duration,
    pub distance:Au,
}
impl StellarBody{
    pub fn calcPosition(&self, sinceStartOfSimulation:Duration) -> Position{
        let orbitTime:i64 = self.orbitTime.num_seconds();
        if orbitTime == 0 {
            // prevents division by 0
            return center;
        }
        // cut off previous orbits
        let cycleProgress:i64 = sinceStartOfSimulation.num_seconds() % orbitTime; // calculate the current orbit progress
        let progressFraction:f64 = (cycleProgress as f64)/(orbitTime as f64);
        // calulate the location
        Position{
            x: progressFraction.sin() * self.distance,
            y: progressFraction.cos() * self.distance
        }
    }
}
pub struct System{
    pub position:Position,
    pub radius:Au, // allows quick filtering
    pub bodies:Vec<StellarBody>,
}
impl System{
    pub fn new(position:Position, bodies:Vec<StellarBody>) -> System{
        let radius = bodies.iter().fold(0.0,|prev,body|->f64{
            let newDist = body.distance;
            if newDist > prev{
                newDist
            }else{
                prev
            }
        });
        System{position:position,bodies:bodies,radius:radius}
    }
}

pub fn create_single_star(name:&'static str)->StellarBody{
    StellarBody{
        name:name,
        orbitTime: Duration::zero(),
        distance:0.0,
    }
}
