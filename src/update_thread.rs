// This file contains the core update thread, ie make sure time runs

use std::thread;
use std::sync::{Arc, RwLock, Mutex};
use state_machine::StateEvent;
use time::Duration;

pub struct Updater{
    pub event:Arc<Mutex<StateEvent>>,
    pub game_time:Arc<RwLock<Duration>>,
    pub controll:Arc<RwLock<UpdateControll>>
}
impl Updater{
    pub fn new(start_event:StateEvent, start_duration:Duration) -> Updater{
        Updater{
            event:Arc::new(Mutex::new(start_event)),
            game_time:Arc::new(RwLock::new(start_duration)),
            controll:Arc::new(RwLock::new(
                UpdateControll{
                    pace_ms:250,
                    running:true,
                    granuality:Duration::weeks,
                    paused:true
                }
            ))
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
        let game_time = self.game_time.clone();
        let controll = self.controll.clone();
        thread::spawn(move|| {
            Updater::run(event, game_time, controll);
        });
    }
    fn run(event:Arc<Mutex<StateEvent>>, game_time:Arc<RwLock<Duration>>, controll:Arc<RwLock<UpdateControll>>){
        use std::time;
        let one_ms = time::Duration::from_millis(1);
        loop{
            if controll.read().expect("reading paused").paused {
                thread::yield_now();
                // just yielding is to little, the thread will still consume 1 core
                thread::sleep(one_ms); 
                continue;
            }
            {
                // at this point we gave up on channels and let locks into our
                // hearts, it actually made things simpler, believe it or not.
                let previous = *game_time.read().expect("poison time");
                let mktimefunc = controll.read().unwrap().granuality;
                *game_time.write().expect("poisned") = previous + mktimefunc(1);
                *event.lock().expect(
                    "posining on set event") = StateEvent::WantsUpdate;
            } // drop the other locks, so we don't keep a lock while waiting
            // about 50~60 is the minimum witout cpu buildup
            // maybe I should detect that and auto throttle back or something
            let pace = time::Duration::from_millis(controll.read().expect("reading pace").pace_ms);
            thread::sleep(pace);
        }
    }
    pub fn toggle_pause(&mut self){
        let ispaused = self.controll.read().expect("reading pause status").paused;
        self.controll.write().expect("writing pause status").paused = !ispaused;
    }
    pub fn set_granuality(&mut self, to:fn(i64)->Duration){
        self.controll.write().expect("writing new granu").granuality = to;
    }
}
// allow the pacing of the game trough this struct
pub struct UpdateControll{
    pub pace_ms:u64,
    pub running:bool, 
    pub granuality:fn(i64)->Duration,
    pub paused:bool,
}
