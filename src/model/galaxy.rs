// This program is a 4x space game.
// Copyright (C) 2016 Jappie Klooster

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.If not, see <http://www.gnu.org/licenses/>.

// this file describes the game model, ie key datastructures for the game play.
// for now we jam just everything in here since model description is pretty
// staight forward, probably gonna do seperate files if this gets bigger than
// 500+ lines or so


use chrono::Duration;
use crate::geometry::*;
use crate::model::colony::Colony;
use std::usize;

// austronomical unit, distance from the earth to the sun. Turns out the milky
// way fits nicely in a signed f64 au if you take earth as 0.0
pub type Au = f64;

// relative to earth
pub type Earths = f64;

#[derive(Clone)]
pub enum BodyClass{
    Rocky(Colony),
    GasGiant,
    Star,
}

#[derive(Clone)]
pub struct StellarBody{
    pub class:BodyClass,
    pub name:&'static str,
    pub orbit_time:Duration,
    pub distance:Au,
    // if you have the body you can modify it in constant time
    pub address:BodyAddress,
}
impl StellarBody{
    pub fn new(class:BodyClass, name:&'static str, orbit:Duration, distance:Au) -> StellarBody{
        StellarBody{
            class:class,
            name:name,
            orbit_time: orbit,
            distance:distance,
            address:unkown_address,
        }
    }
    // create a stellar body with properties of earth (in game)
    pub fn new_earthlike(name:&'static str) -> StellarBody{
        use crate::model::colony::*;
        StellarBody::new(
            BodyClass::Rocky(
                Colony::new_inhabited(
                    0,
                    1.0,
                    Population::new(
                        7456000000
                    )
                )
            ),
            name,
            Duration::days(365),
            1.0
        )
    }
    pub fn create_single_star(name:&'static str)->StellarBody{
        StellarBody::new(BodyClass::Star, name, Duration::zero(), 0.0)
    }
    pub fn calc_position(&self, since_start_of_simulation:&Duration) -> Position{
        calc_orbit(&self.orbit_time, self.distance, since_start_of_simulation)
    }
    pub fn get_colony<'b>(&'b self)->Option<&'b Colony>{
        match self.class{
            BodyClass::Rocky(ref colony) => Some(&colony),
            _ => None
        }
    }
}
pub fn calc_orbit(orbit_duration:&Duration, orbit_distance:Au, time:&Duration) -> Position{
    let orbit_time:i64 = orbit_duration.num_milliseconds();
    if orbit_time == 0 {
        // prevents division by 0
        return center;
    }
    // cut off previous orbits
    let cycle_pogress:i64 = time.num_milliseconds() % orbit_time; // calculate the current orbit progress
    use std::f64::consts;
    let progress_fraction:f64 = ((cycle_pogress as f64)/(orbit_time as f64)) * consts::PI * 2.0;
    // calulate the location
    Position{
        x: progress_fraction.sin() * orbit_distance,
        y: progress_fraction.cos() * orbit_distance 
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
//TODO: perhaps we can implement a custom hash that abuses knowledge of
// systemsizes to make a simple addition sum of it. (this knowledge should
// be available at compile time, as long as we don't generate anything randomly)
pub struct BodyAddress{
    pub system_id:usize,
    pub planet_id:usize,
    // TODO: moon id? maybe as an option?
}
const unkown_address:BodyAddress = BodyAddress{system_id:usize::MAX,planet_id:usize::MAX};

#[derive(Clone)]
pub struct System{
    pub used_space:Disk,
    pub bodies:Vec<StellarBody>,
}
impl System{
    pub fn new(position:Position, bodies:Vec<StellarBody>) -> System{
        let radius = bodies.iter().fold(0.0,|prev,body|->f64{
            let new_dist = body.distance;
            if new_dist > prev{
                new_dist
            }else{
                prev
            }
        });
        System{
            used_space: Disk{
                position:position,
                radius:radius
            },
            bodies:bodies
        }
    }
}

#[derive(Clone)]
pub struct Galaxy{
    pub systems:Vec<System>
}
impl Galaxy{
    pub fn new(systems:Vec<System>) -> Galaxy{
        Galaxy{
            systems:systems
        }
    }
}
use std::ops::{Index,IndexMut};
impl Index<BodyAddress> for Galaxy {
    type Output = StellarBody;
    fn index(&self, index: BodyAddress) -> &Self::Output{
        &self.systems[index.system_id].bodies[index.planet_id]
    }
}

impl IndexMut<BodyAddress> for Galaxy {
    fn index_mut<'a>(&'a mut self, index: BodyAddress) -> &'a mut StellarBody {
        &mut self.systems[index.system_id].bodies[index.planet_id]
    }
}
