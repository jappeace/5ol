// This file contains the core update thread, ie make sure time runs

use std::thread;
use std::sync::{Arc, RwLock, Mutex};
use state_machine::StateEvent;
use time::Duration;

pub struct Updater{
    pub event:Arc<Mutex<StateEvent>>,
    pub game_time:Arc<RwLock<Duration>>,
}
impl Updater{
    pub fn new(start_event:StateEvent, start_duration:Duration) -> Updater{
        Updater{
            event:Arc::new(Mutex::new(start_event)),
            game_time:Arc::new(RwLock::new(start_duration))
        }
    }
    pub fn start(&self) {
        let event = self.event.clone();
        let game_time = self.game_time.clone();
        thread::spawn(move|| {
            Updater::run(event, game_time);
        });
    }
    fn run(event:Arc<Mutex<StateEvent>>, game_time:Arc<RwLock<Duration>>){
        loop{
            // at this point we gave up on channels and let locks into our
            // hearts, it actually made things simpler, believe it or not.
            let previous = *game_time.read().expect("poison time");
            *game_time.write().expect("poisned") = previous + Duration::weeks(1);
            *event.lock().expect(
                "posining on set event") = StateEvent::WantsUpdate;
            use std::time;
            // about 50~60 is the minimum witout cpu buildup
            // maybe I should detect that and auto throttle back or something
            thread::sleep(time::Duration::from_millis(100)); 
        }
    }
}
