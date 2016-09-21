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
pub type Au = f64;

use time::Duration;
use geometry::*;
use petgraph::graph::NodeIndex;

pub struct StellarBody{
    pub name:&'static str,
    pub orbit_time:Duration,
    pub distance:Au,
    // conrod uses an id to keep track of widgets,
    // however since the stellar bodies are generated we need to decide these
    // on the fly, see: http://docs.piston.rs/conrod/conrod/widget/id/type.Id.html
    // the initial approach just generate a new one forever, but I think that
    // is just a memory leak.
    pub view_id:Option<NodeIndex<u32>>,
}
impl StellarBody{
    pub fn new(name:&'static str, orbit:Duration, distance:Au) -> StellarBody{
        StellarBody{
            name:name,
            orbit_time: orbit,
            distance:distance,
            view_id:None
        }
    }
    pub fn create_single_star(name:&'static str)->StellarBody{
        StellarBody::new(name, Duration::zero(), 0.0)
    }
    pub fn calc_position(&self, since_start_of_simulation:&Duration) -> Position{
        let orbit_time:i64 = self.orbit_time.num_seconds();
        if orbit_time == 0 {
            // prevents division by 0
            return center;
        }
        // cut off previous orbits
        let cycle_pogress:i64 = since_start_of_simulation.num_seconds() % orbit_time; // calculate the current orbit progress
        use std::f64::consts;
        let progress_fraction:f64 = ((cycle_pogress as f64)/(orbit_time as f64)) * consts::PI * 2.0;
        // calulate the location
        Position{
            x: progress_fraction.sin() * self.distance,
            y: progress_fraction.cos() * self.distance
        }
    }
}
pub struct BodyAdress{
    pub system_id:usize,
    pub planet_id:usize,
    // TODO: moon id? maybe as an option?
}
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

// top level datastructure, all other models should be attached to this.
// having this allows us to transfer ownership of the current game progress
// between "states".
pub struct GameModel{
    pub galaxy:Vec<System>,
    pub player:Player,
}
impl GameModel{
    pub fn new(systems:Vec<System>) -> GameModel{
        GameModel{
            galaxy:systems,
            player:Player{
                money:0
            }
        } 
    }
    pub fn get_body(&self, address:&BodyAdress) -> &StellarBody{
        &self.galaxy[address.system_id].bodies[address.planet_id]
    }
}
pub struct Player{
    money:i32
}
