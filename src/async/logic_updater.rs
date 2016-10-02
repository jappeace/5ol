// This file contains the core update thread, ie make sure time update_natures

use std::thread;
use std::time;
use std::sync::{Arc, RwLock, Mutex};
use state::state_machine::StateEvent;
use time::Duration;

use model::GameModel;
use async::model_access::{ModelAccess, Change};
use async::thread_status::{ThreadControll, Status};
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct Updater{
    pub controll:ThreadControll,
    pub granuality:Arc<RwLock<fn(i64)->Duration>>,
    model_writer:ModelAccess
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
    pub fn enqueue(&self, change:Change){
        self.model_writer.enqueue(change);
    }
    pub fn model_access(&self) -> ModelAccess{
        self.model_writer.clone()
    }
    pub fn read_model(&self) -> GameModel{
        self.model_writer.read_model()
    }
    pub fn start(&mut self) {
        self.model_writer.start();
        let model = self.model_writer.clone();
        let granuality = self.granuality.clone();

        self.controll.execute_async(move ||{
            Updater::update_nature(model.clone(), granuality.clone());
        })
    }
    fn update_nature(model_writer:ModelAccess, granuality:Arc<RwLock<fn(i64)->Duration>>){
        // at this point we gave up on channels and let locks into our
        // hearts, it actually made things simpler, believe it or not.
        let previous = model_writer.game_model.read().expect("poison time").time;
        let mktimefunc = granuality.read().unwrap();
        model_writer.enqueue(Change::Time(previous + mktimefunc(1)));
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
