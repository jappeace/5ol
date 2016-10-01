// This file contains the core update thread, ie make sure time update_natures

use std::thread;
use std::time;
use std::sync::{Arc, RwLock, Mutex};
use state::state_machine::StateEvent;
use time::Duration;

use model::GameModel;
use async::thread_status::{ThreadControll, Status};
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct Updater{
    pub game_model:Arc<RwLock<GameModel>>,
    pub controll:ThreadControll,
    pub granuality:Arc<RwLock<fn(i64)->Duration>>,
    pub change_queue:Sender<Change>
}
impl Updater{
    pub fn new(start_model:Arc<RwLock<GameModel>>, granuality:fn(i64)->Duration) -> Updater{
        let mut controll = ThreadControll::new();
        controll.set_status(Status::Paused);
        controll.set_pace(250);
        let (cq,cr) = channel();
        Updater{
            game_model:start_model,
            controll:controll,
            granuality:Arc::new(RwLock::new(granuality)),
            change_queue:cq
        }
    }
    pub fn start(&mut self) {
        let game_model = self.game_model.clone();
        let granuality = self.granuality.clone();
        let (cq,user_changes) = channel();
        self.change_queue = cq;

        self.controll.execute_async(move ||{
            while let Ok(message) = user_changes.try_recv(){
                Updater::update_interaction(game_model.clone(), &message);
            }
            Updater::update_nature(game_model.clone(), granuality.clone());
        })
    }
    fn update_interaction(game_model:Arc<RwLock<GameModel>>, change:&Change){
        match change{
            &Change::BodyViewID(address, changeto) => {
                let mut body = game_model.read().expect("it").get_body(&address).clone();
                body.view_id = changeto;
                game_model.write().expect("it").set_body(&address, body);
            }       
        }
    }
    fn update_nature(game_model:Arc<RwLock<GameModel>>, granuality:Arc<RwLock<fn(i64)->Duration>>){
        // at this point we gave up on channels and let locks into our
        // hearts, it actually made things simpler, believe it or not.
        let previous = game_model.read().expect("poison time").time;
        let mktimefunc = granuality.read().unwrap();
        game_model.write().expect("poisned").time = previous + mktimefunc(1);
    }
    pub fn set_granuality(&mut self, to:fn(i64)->Duration){
        *self.granuality.write().expect("writing new granu") = to;
    }
}

use petgraph::graph::NodeIndex;
use model::BodyAddress;
pub enum Change{
    BodyViewID(BodyAddress, Option<NodeIndex<u32>>)
}
