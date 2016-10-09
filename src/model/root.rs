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


// This is the root model or model abstract, the root of the state of the program

use chrono::Duration;
use model::galaxy::{System, BodyAddress};
use model::ship::Ship;
use std::usize;

// top level datastructure, all other models should be attached to this.
// having this allows us to transfer ownership of the current game progress
// between "states".
#[derive(Clone)]
pub struct GameModel{
    pub galaxy:Vec<System>,
    pub players:Vec<Player>,
    pub ships:Vec<Ship>,
    pub time:Duration
}
impl GameModel{
    pub fn new(systems:Vec<System>) -> GameModel{
        let addressed = (0..).zip(systems).map(|(s_i,sys)|{
            let mut newsys = sys.clone();
            let newbodies = (0..).zip(sys.bodies).map(|(p_i, body)|{
                let mut newbody = body.clone();
                newbody.address = BodyAddress{system_id:s_i, planet_id:p_i};
                newbody
            }).collect();
            newsys.bodies = newbodies;
            newsys
        }).collect();
        GameModel{
            galaxy:addressed,
            players:vec![Player{
                money:0,
                id:0
            }],
            ships:Vec::new(),
            time:Duration::zero()
        } 
    }
}

#[derive(Clone)]
pub struct Player{
    pub money:i64,
    pub id:usize
}
