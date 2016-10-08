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


// This file contains a structure that is responsable for updating the model.
// this is done on a dedicated thread that blocks untill changes enter it.
// all model writes should go trough this structure, each model should have
// a respective singular writer.

// The main idea behind this is to prevent race conditions and group certain
// updates together to prevent readers from getting weird states, such as for
// example a time update should also include a resource tick. So that you never
// get increased resources without increased time.
// It also should prevent cheating by doing model checks, such as, "is this
// change even allowed". Because the clients may have been working on an older
// state where things still could've worked.

use std::thread;
use std::sync::{Arc, RwLock, RwLockWriteGuard, RwLockReadGuard};
use chrono::Duration;

use model::root::GameModel;
use model::galaxy::{BodyAddress,BodyClass};
use model::colony::carrying_capacity_earth;
use petgraph::graph::NodeIndex;

use async::thread_status::{ThreadControll, Status};
use std::sync::mpsc::{channel, Sender};


#[derive(Clone)]
pub struct ModelAccess{
    controll:ThreadControll,
    pub game_model:Arc<RwLock<GameModel>>,
    pub change_queue:Sender<Change>
}
impl ModelAccess{
    pub fn new(start_model:Arc<RwLock<GameModel>>) -> ModelAccess{
        let mut controll = ThreadControll::new();
        controll.set_status(Status::Aborted);
        controll.set_pace(0);
        let (cq,_) = channel();
        ModelAccess{
            game_model:start_model,
            controll:controll,
            change_queue:cq
        }
    }
    pub fn start(&mut self) {
        if self.controll.get_status() != Status::Aborted {
            panic!("already started");
        }
        self.controll.set_status(Status::Executing);

        let game_model = self.game_model.clone();
        let (cq,user_changes) = channel();
        self.change_queue = cq;

        self.controll.execute_async(move ||{
            match user_changes.recv(){
                Ok(message) => ModelAccess::write(game_model.clone(), &message),
                _ => {
                    // it means that all senders are de-allocated
                    // therefore this thread became useless and the easiest way
                    // of dealing with this is crashing it.
                    panic!("otherside hung up");
                }
            }
        })
    }
    pub fn stop(&mut self){
        let sender = self.change_queue.clone();
        // prevent a deadlock by flushing the thread with messages
        thread::spawn(move ||{
            while let Ok(_) = sender.send(Change::Nothing) {
                thread::yield_now();
            }
            // otherside hung up, this is what we wanted, this thread can die now
        });
        self.controll.stop();
        println!("stopped access");
    }
    pub fn enqueue(&self, change:Change){
        if let Ok(_) = self.change_queue.send(change){
            return;
        }
        panic!("sending failed");
    }
    pub fn copy_model(&self) -> GameModel{
        self.read_lock_model().clone()
    }
    pub fn read_lock_model(&self) -> RwLockReadGuard<GameModel>{
        if let Ok(gaurd) = self.game_model.read(){
            return gaurd;
        }
        println!("poisned, try again");
        self.read_lock_model()
    }
    fn write(game_model:Arc<RwLock<GameModel>>, change:&Change){
        match change{
            &Change::BodyViewID(address, changeto) => {
                let mut body = address.get_body(&game_model.read().expect("it").galaxy).clone();
                body.view_id = changeto;
                address.set_body(&mut game_model.write().expect("it").galaxy, body);
            }       
            &Change::Time(increase) => ModelAccess::resource_tick(game_model.write().expect("it"), increase),
            &Change::Nothing => {}
        }
    }
    fn resource_tick(mut game_model:RwLockWriteGuard<GameModel>, interval:Duration){
        game_model.time = game_model.time + interval;
        let changes:Vec<(BodyAddress,i64,f64)> = game_model.galaxy.iter()
            .flat_map(|x| x.bodies.iter().filter_map(|cur| {
                    match &cur.class {
                        &BodyClass::Rocky(ref h) => {
                            if let Some(pop) = h.population.clone(){
                                Some((
                                    cur.address,
                                    pop.calc_head_increase(
                                        (h.size*carrying_capacity_earth) as i64,
                                        interval
                                    ),
                                    pop.calc_tax_over(interval),
                                ))
                            }else{
                                None
                            }
                        },
                        _ =>{ None}
                    }
                })
            ).collect();
        for change in changes{
            let mut newbody = change.0.get_body(&game_model.galaxy).clone();
            newbody.class = if let BodyClass::Rocky(mut habitat) = newbody.class{
                habitat.population = habitat.population.map(|x| {
                    game_model.players[x.owner].money += change.2 as i64;
                    x.change_headcount(change.1)  
                });
                BodyClass::Rocky(habitat)
            }else{
                newbody.class
            };
            change.0.set_body(&mut game_model.galaxy, newbody);
        }
    }
}

#[derive(Debug)]
pub enum Change{
    BodyViewID(BodyAddress, Option<NodeIndex<u32>>),
    Time(Duration),
    Nothing // usefull for dealing with controll changes (ie thread abort)
}
