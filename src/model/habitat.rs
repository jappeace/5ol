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


// this file models the places where people live
use chrono::Duration;
use model::galaxy::Earths;
use std::usize;

#[derive(Clone)]
pub struct Habitat{
    pub size:Earths,
    // not colonized, no pop
    pub population:Option<Population>
}
impl Habitat{
    pub fn new_empty(size:Earths)->Habitat{
        Habitat{
            population:None,
            size:size
        }
    }
    pub fn new_inhabited(surface:Earths, population:Population)->Habitat{
        let mut result = Habitat::new_empty(surface);
        result.population = Some(population);
        result
    }
}

#[derive(Clone)]
pub struct Population{
    pub owner:usize, // playerid
    pub head_count:i64,
    pub tax:f64 // annual tax pp
}
impl Population{
    pub fn new(owner:usize,head_count:i64)->Population{
        Population{
            owner:owner,
            head_count:head_count,
            tax:0.1,
        } 
    }
    pub fn change_headcount(mut self, by:i64) -> Self{
        println!("heads {}", self.head_count);
        self.head_count += by;
        self
    }
    pub fn calc_tax_over(&self, duration:Duration) -> f64{
        let fraction = duration.num_milliseconds() as f64 / Duration::days(1).num_milliseconds() as f64;
        self.tax * (self.head_count as f64) * fraction
    }
    pub fn calc_head_increase(&self, carrying_capacity:i64, duration:Duration) -> i64{
        // lets say we reach carrying capacity in 50 years
        let cc_fraction = (self.head_count as f64) / (carrying_capacity as f64);

        if cc_fraction > 1.0 {
            let week = Duration::weeks(1).num_milliseconds() as f64;
            let death_time = (duration.num_milliseconds() as f64)/week;
            let deaths_fraction = ((cc_fraction-1.0)*death_fraction_per_week) * death_time;
            return -(deaths_fraction * (self.head_count as f64)) as i64;
        }
        let thirthy_year = Duration::weeks(52*30).num_milliseconds() as f64;
        let time_fraction = duration.num_milliseconds() as f64 / thirthy_year;
        let fertile_pop = (self.head_count as f64) * fertile_female_fraction;
        //TODO: limit growth for small population sizes
        // ie, figure female count, times amount of babies they get over duration
        // only neccesary for postive growth
        (time_fraction * fertile_pop * ((1.0-cc_fraction)*growth_boost)) as i64
    }
}
pub const carrying_capacity_earth:f64 = 10000000000.0;
const fertile_female_fraction:f64 = 0.5*0.3;
const growth_boost:f64 = 1.0;

const death_fraction_per_week:f64 = 0.1;
