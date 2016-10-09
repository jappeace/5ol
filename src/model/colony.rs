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
use model::galaxy::{Earths,BodyAddress };
use model::root::{GameModel};
use std::usize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Colony{
    pub size:Earths,
    pub owner:Option<usize>, // playerid
    // not colonized, no pop
    pub population:Option<Population>,
    pub construction_queue:Vec<Construction>,
}
impl Colony{
    pub fn new_empty(size:Earths)->Colony{
        Colony{
            population:None,
            owner:None,
            size:size,
            construction_queue:Vec::new()
        }
    }
    pub fn new_inhabited(owner:usize, surface:Earths, population:Population)->Colony{
        let mut result = Colony::new_empty(surface);
        result.population = Some(population);
        result.owner = Some(owner);
        result
    }
    pub fn construction_tick(&mut self, work_time:Duration) -> Vec<AConstructable>{
        let not_done_yet = Vec::new();
        if work_time < Duration::zero(){
            return not_done_yet;
        }
        let construct = self.construction_queue.pop();
        match construct {
            Some(mut job) => if let Some(remainder) = job.work_on(work_time){
                let mut result = self.construction_tick(remainder);
                result.push(job.constructable);
                result
            }else{
                println!("not finished ship");
                self.construction_queue.push(job);
                not_done_yet
            },
            _ => not_done_yet
        }
    }
}

pub type AConstructable = Arc<Constructable + Send + Sync>;
#[derive(Clone)]
pub struct Construction{
    pub progress:Duration,
    pub constructable: AConstructable 
}
impl Construction{
    pub fn new(on_complete:AConstructable)-> Construction{
        Construction{
            progress:Duration::zero(),
            constructable:on_complete
        }
    }
    // work on construction, returns some duration if progress exceeds needed
    pub fn work_on(&mut self, time_passed:Duration) -> Option<Duration>{
        self.progress = self.progress + time_passed;
        let needed = self.constructable.work_needed();
        if needed < self.progress {
            Some(self.progress - needed)
        }else{
            None
        }
    }
}
#[allow(unused_variables)]
pub trait Constructable{
    fn on_complete(&self, model:&mut GameModel, contructor_address:&BodyAddress)->(){}
    fn work_needed(&self) -> Duration{Duration::weeks(4)}
    fn price(&self) -> i64;
}
#[derive(Clone)]
pub struct Population{
    pub head_count:i64,
    pub tax:f64 // annual tax pp
}
impl Population{
    pub fn new(head_count:i64)->Population{
        Population{
            head_count:head_count,
            tax:0.1,
        } 
    }
    pub fn change_headcount(mut self, by:i64) -> Self{
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
