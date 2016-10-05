// This is the GUI pulser, every N seconds we allow the frame to be redrawn
// this is to prevent an issue with conrod which causes 100% usage on the
// ui thread.

use std::sync::{Arc, Mutex};
use state::state_machine::StateEvent;

use async::thread_status::ThreadControll;

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
        self.controll.execute_async(move ||{
            *event.lock().expect("posining on set event") = StateEvent::WantsUpdate;
        });
    }
}
