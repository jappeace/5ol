// This file is the logic behind the time controlls.

use std::sync::{Arc, RwLock};
use time::Duration;

use model::GameModel;
use async::model_access::{ModelAccess, Change};
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
        let mktimefunc = granuality.read().unwrap();
        model_writer.enqueue(Change::Time(mktimefunc(1)));
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
