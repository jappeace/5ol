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

use crate::model::GameModel;
use super::model_access::{ModelAccess, Change};
use super::thread_status::{ThreadControll, Status};
use std::sync::mpsc::Sender;

pub struct Updater{
    pub controll:ThreadControll,
    pub granuality:Arc<RwLock<fn(i64)->Duration>>,
    pub change_queue:Option<Sender<Change>>,
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
            model_writer:ModelAccess::new(start_model),
            change_queue:None
        }
    }
    pub fn start(&mut self) {
        let sender = self.model_writer.start();
        self.change_queue = Some(sender.clone());
        let model = self.model_writer.game_model.clone();
        let granuality = self.granuality.clone();

        self.controll.execute_logic(move ||{
            Updater::update_nature(model.clone(), sender.clone(), granuality.clone());
        })
    }
    pub fn enqueue(&self, change:Change){
        self.change_queue.clone().map(|x| x.send(change));
    }
    #[allow(unused_variables)] // need that lock
    fn update_nature(model_writer:Arc<RwLock<GameModel>>, sender:Sender<Change>, granuality:Arc<RwLock<fn(i64)->Duration>>){
        // obtain read lock to prevent going faster than the writer at speed 0
        let lock = model_writer.read();
        let mktimefunc = granuality.read().unwrap();
        sender.send(Change::Time(mktimefunc(1))).expect("other side hung up");
    }
    pub fn set_granuality(&mut self, to:fn(i64)->Duration){
        *self.granuality.write().expect("writing new granu") = to;
    }
    pub fn stop(&mut self){
        self.controll.stop();
        println!("stopped controll");
    }
}
