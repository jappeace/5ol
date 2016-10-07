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


// This file is the logic behind the time controlls.

use std::sync::{Arc, RwLock};
use chrono::Duration;

use model::{GameModel, BodyClass, carrying_capacity_earth};
use async::model_access::{ModelAccess, Change, HabitatTick};
use async::thread_status::{ThreadControll, Status};

pub struct Updater{
    pub controll:ThreadControll,
    pub granuality:Arc<RwLock<fn(i64)->Duration>>,
    pub model_writer:ModelAccess
}
impl Updater{
    pub fn new(start_model:Arc<RwLock<GameModel>>, granuality:fn(i64)->Duration) -> Updater{
        let mut controll = ThreadControll::new();
        controll.set_status(Status::Paused);
        controll.set_pace(250);
        Updater{
            controll:controll,
            granuality:Arc::new(RwLock::new(granuality)),
            model_writer:ModelAccess::new(start_model)
        }
    }
    pub fn start(&mut self) {
        self.model_writer.start();
        let model = self.model_writer.clone();
        let granuality = self.granuality.clone();

        self.controll.execute_async(move ||{
            Updater::update_nature(model.clone(), granuality.clone());
        })
    }
    #[allow(unused_variables)] // need that lock
    fn update_nature(model_writer:ModelAccess, granuality:Arc<RwLock<fn(i64)->Duration>>){
        // obtain read lock to prevent going faster than the writer at speed 0
        let lock = model_writer.read_lock_model();
        let interval = granuality.read().unwrap()(1);
        let changes:Vec<HabitatTick> = lock.galaxy.iter()
            .flat_map(|x| x.bodies.iter().filter_map(|cur| {
                match &cur.class {
                    &BodyClass::Rocky(ref h) => {
                        if let Some(pop) = h.population.clone(){
                            Some(
                                HabitatTick{
                                    address:cur.address,
                                    pop_change:pop.calc_head_increase(
                                        (h.size*carrying_capacity_earth) as i64,
                                        interval
                                    ),
                                    money_change:pop.calc_tax_over(interval)
                                }
                            )
                        }else{
                            None
                        }
                    },
                    _ =>{ None}
                }
            })
            ).collect();
        model_writer.enqueue(Change::Time(interval, changes));
    }
    pub fn set_granuality(&mut self, to:fn(i64)->Duration){
        *self.granuality.write().expect("writing new granu") = to;
    }
    pub fn stop(&mut self){
        self.controll.stop();
        println!("stopped controll");
        self.model_writer.stop();
        println!("stopped updater")
    }
}
