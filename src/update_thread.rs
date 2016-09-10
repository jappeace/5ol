// This file contains the core update thread, ie make sure time update_times

use std::thread;
use std::time;
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
                    is_updating_timing:true,
                    granuality:Duration::weeks,
                    is_paused:true
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
        let game_time = self.game_time.clone();
        let controll = self.controll.clone();
        thread::spawn(move|| {
            Updater::update_time(game_time, controll);
        });
        let event = self.event.clone();
        let controll = self.controll.clone();
        thread::spawn(move|| {
            Updater::update_gui(event, controll);
        });
    }
    fn read_status(controll:&Arc<RwLock<UpdateControll>>) -> ThreadStatus{
        if !controll.read().expect("reading is_updating_timing").is_updating_timing{
            return ThreadStatus::Aborted;
        }
        if controll.read().expect("reading is_paused").is_paused {
            return ThreadStatus::Paused;
        }
        ThreadStatus::Executing
    }
    fn update_gui(event:Arc<Mutex<StateEvent>>, controll:Arc<RwLock<UpdateControll>>){
        let hundert_ms = time::Duration::from_millis(16);
        loop{
            match Updater::read_status(&controll) {
                ThreadStatus::Aborted => break,
                ThreadStatus::Paused =>{
                    Updater::sleep();
                    continue;
                }
                ThreadStatus::Executing => {}
            }
            *event.lock().expect(
                "posining on set event") = StateEvent::WantsUpdate;
            // about 50~60 is the minimum witout cpu buildup from conrod,
            // we keep a safe 100ms, this runs on a seperate thread from the
            // main updater, so on speed 5 it can go wild, we just don't see
            // all updates
            // see https://github.com/PistonDevelopers/conrod/issues/814
            thread::sleep(hundert_ms);
        }
    }
    fn sleep(){
        let one_ms = time::Duration::from_millis(1);
        thread::yield_now();
        // just yielding is to little, the thread will still consume 1 core
        thread::sleep(one_ms); 
    }
    fn update_time(game_time:Arc<RwLock<Duration>>, controll:Arc<RwLock<UpdateControll>>){
        loop{
            match Updater::read_status(&controll) {
                ThreadStatus::Aborted => break,
                ThreadStatus::Paused =>{
                    Updater::sleep();
                    continue;
                }
                ThreadStatus::Executing => {
                    // at this point we gave up on channels and let locks into our
                    // hearts, it actually made things simpler, believe it or not.
                    let previous = *game_time.read().expect("poison time");
                    let mktimefunc = controll.read().unwrap().granuality;
                    *game_time.write().expect("poisned") = previous + mktimefunc(1);
                }
            }
            let pace = time::Duration::from_millis(controll.read().expect("reading pace").pace_ms);
            thread::sleep(pace);
        }
    }
    pub fn toggle_pause(&mut self){
        let isis_paused = self.controll.read().expect("reading pause status").is_paused;
        self.controll.write().expect("writing pause status").is_paused = !isis_paused;
    }
    pub fn set_granuality(&mut self, to:fn(i64)->Duration){
        self.controll.write().expect("writing new granu").granuality = to;
    }
}
// allow the pacing of the game trough this struct
pub struct UpdateControll{
    pub pace_ms:u64,
    pub is_updating_timing:bool, 
    pub granuality:fn(i64)->Duration,
    pub is_paused:bool,
}

enum ThreadStatus{
    Aborted,
    Paused,
    Executing
}
