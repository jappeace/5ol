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


// This is the GUI pulser, every N seconds we allow the frame to be redrawn
// this is to prevent an issue with conrod which causes 100% usage on the
// ui thread.

use std::sync::{Arc, Mutex};
use crate::state::state_machine::StateEvent;

use super::thread_status::ThreadControll;

pub struct Pulser{
    pub event:Arc<Mutex<StateEvent>>,
    pub controll:ThreadControll
}
impl Pulser{
    pub fn new(start_event:StateEvent) -> Pulser{
        let mut controll = ThreadControll::new();
        // about 50~60 is the minimum witout cpu buildup from conrod,
        // we keep a safe 100ms, this runs on a seperate thread from the
        // main updater, so on speed 5 it can go wild, we just don't see
        // all updates
        // see https://github.com/PistonDevelopers/conrod/issues/814
        controll.set_pace(16);
        Pulser{
            event:Arc::new(Mutex::new(start_event)),
            controll:controll
        }
    }
    pub fn get_event(&self) -> StateEvent{
        let mut shared = self.event.lock().expect("poisining on read event");
        let result = *shared;
        *shared = StateEvent::Idle;
        result
    }
    pub fn start(&self) {
        let event = self.event.clone();
        self.controll.execute_logic(move ||{
            *event.lock().expect("posining on set event") = StateEvent::WantsUpdate;
        });
    }
}
