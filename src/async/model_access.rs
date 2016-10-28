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

use model::root::{GameModel, PlayerID};
use model::galaxy::{BodyAddress,BodyClass};
use model::ship::ShipID;
use model::colony::{AConstructable,Construction, carrying_capacity_earth};
use petgraph::graph::NodeIndex;

use async::thread_status::{ThreadControll, Status};
use std::sync::mpsc::{channel, Sender};


#[derive(Clone)]
pub struct ModelAccess{
    pub game_model:Arc<RwLock<GameModel>>
}
impl ModelAccess{
    pub fn new(start_model:Arc<RwLock<GameModel>>) -> ModelAccess{
        let mut controll = ThreadControll::new();
        controll.set_status(Status::Aborted);
        controll.set_pace(0);
        ModelAccess{
            game_model:start_model,
        }
    }
    pub fn start(&mut self) -> Sender<Change> {
        println!("start access");

        let game_model = self.game_model.clone();
        let (sender,receiver) = channel();

        thread::spawn(move ||{
            let mut running = true;
            while running{
                match receiver.recv(){
                    Ok(message) => ModelAccess::write(game_model.clone(), &message),
                    _ => {
                        running = false
                    }
                }
            }
        });
        sender
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
        match *change{

            Change::BodyViewID(address, changeto) => {
                let mut body = &mut game_model.write().expect("it").galaxy[address];
                body.view_id = changeto;
            }       

            Change::ShipViewID(id,changeto) =>
                game_model.write().expect("it").ships[id].view = changeto,

            Change::Construct(ref constructable, address) =>{
                let mut body = &mut game_model.write().expect("it").galaxy[address];
                let class = body.class.clone();
                body.class = if let BodyClass::Rocky(mut colony) = class {
                    colony.construction_queue.push(Construction::new(constructable.clone()));
                    BodyClass::Rocky(colony)
                }else{
                    class
                };
            }

            Change::Time(increase) => ModelAccess::resource_tick(game_model.write().expect("it"), increase),

            Change::StopModifications => {
                panic!("done"); // works best
            }

            Change::Select(player, ref selected) => {
                game_model.write().expect("it").players[player].selected = selected.clone();
            }
        }
    }
    fn resource_tick(mut game_model:RwLockWriteGuard<GameModel>, interval:Duration){
        game_model.time = game_model.time + interval;
        let changes:Vec<(BodyAddress,i64,Option<(usize,i64)>)> = game_model.galaxy.systems.iter()
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
                                    // no owner, no tax change
                                    h.owner.map_or(None, |x| Some((x,pop.calc_tax_over(interval) as i64))),
                                ))
                            }else{
                                None
                            }
                        },
                        _ =>{ None}
                    }
                })
            ).collect();
        let mut constructions:Vec<(BodyAddress, AConstructable)> = Vec::new();
        for change in changes{
            let mut subject = game_model.galaxy[change.0].clone();
            subject.class = if let BodyClass::Rocky(mut habitat) = subject.class.clone(){
                change.2.map(|x| game_model.players[x.0].money += x.1);
                habitat.population = habitat.population.map(|x| {
                    x.change_headcount(change.1)  
                });
                constructions.append(
                    &mut habitat.construction_tick(interval)
                        .into_iter().map(|x| (subject.address, x)).collect()
                );
                BodyClass::Rocky(habitat)
            }else{
                subject.class
            };
            game_model.galaxy[change.0] = subject;
        }
        for construction in constructions{
            construction.1.on_complete(&mut game_model, &construction.0);
        }
    }
}

pub enum Change{
    BodyViewID(BodyAddress, Option<NodeIndex<u32>>),
    ShipViewID(usize,Option<NodeIndex<u32>>),
    Construct(AConstructable, BodyAddress),
    Select(PlayerID, Vec<ShipID>),
    Time(Duration),
    StopModifications // usefull for dealing with controll changes (ie thread abort)
}
