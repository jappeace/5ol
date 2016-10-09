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


// this file models ships in the galaxy, the main form of units
use petgraph::graph::NodeIndex;

use geometry::Position;
use model::galaxy::{calc_orbit, BodyAddress, System, Au};
use model::colony::Constructable;
use model::root::GameModel;
use chrono::Duration;
#[derive(Clone)]
pub struct Ship{
    owner:usize, // playerid
    pub id:usize,
    ship_price:i64,
    pub movement:Movement,
    pub view:Option<NodeIndex<u32>>
}
impl Ship{
    pub fn new(
        owner:usize,
        ship_price:i64,
        construct_location:BodyAddress,
    ) -> Ship{
        use std::usize::MAX;
        Ship{
            owner:owner,
            id:MAX, // make sure it'll crash if not assigned
            ship_price:ship_price,
            movement:Movement::Orbit(construct_location),
            view:None
        }
    }
}
impl Constructable for Ship{
    fn on_complete(&self, model:&mut GameModel, _:&BodyAddress)->(){
        print!("completed");
        let mut result = self.clone();
        result.id = model.ships.len();
        model.ships.push(result);
    }
    fn price(&self) -> i64{
        return self.ship_price;
    }
}

#[derive(Clone)]
pub enum Movement{
    Vector(Duration, Position, Velocity),
    Orbit(BodyAddress)
}
impl Movement{
    pub fn calc_position(&self, time:&Duration, galaxy:&Vec<System>)->Position{
        match self {
            &Movement::Vector(start_time, pos,ref vel) => pos+vel.calc_movement(&(time.clone() - start_time)),
            &Movement::Orbit(address) => {
                let body = address.get_body(galaxy);
                body.calc_position(time) + calc_orbit(
                    &Duration::hours(2),
                    ship_orbit_distance,
                    time
                )
            }
        }
    }
}
const ship_orbit_distance:Au = 0.000_000_000_668_449_198;
#[derive(Clone)]
pub struct Velocity{
    direction:f64, // rads
    speed:f64 // au/s
}
impl Velocity{
    fn calc_movement(&self, time:&Duration) -> Position{
        let millis = time.num_milliseconds() as f64 / 1000.0;
        Position::new(
            self.speed*self.direction.cos() * millis,
            self.speed*self.direction.sin() * millis
        )
    }
}
