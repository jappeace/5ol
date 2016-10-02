// This file contains a structure that is responsable for updating the model.
// this is done on a dedicated thread that blocks untill changes enter it.
// all model writes should go trough this structure, each model should have
// a respective singular writer.

// However keep in mind that this structure only should do simple assignments
// the computation should already be done in at this point and we use this
// structure to make writes determinstic, therefore it also provides a snapshot
// read feature.

use std::thread;
use std::time;
use std::sync::{Arc, RwLock, Mutex};
use state::state_machine::StateEvent;
use time::Duration;

use model::GameModel;
use async::thread_status::{ThreadControll, Status};
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct ModelAccess{
    controll:ThreadControll, // private since controll should go trough Change messages
    pub game_model:Arc<RwLock<GameModel>>,
    pub change_queue:Sender<Change>
}
impl Clone for ModelAccess{
    fn clone(&self) -> Self{
        ModelAccess{
            controll:self.controll.clone(),
            game_model:self.game_model.clone(),
            change_queue:self.change_queue.clone()
        }
    }
}
impl ModelAccess{
    pub fn new(start_model:Arc<RwLock<GameModel>>) -> ModelAccess{
        let mut controll = ThreadControll::new();
        controll.set_status(Status::Aborted);
        controll.set_pace(0);
        let (cq,cr) = channel();
        ModelAccess{
            game_model:start_model,
            controll:controll,
            change_queue:cq
        }
    }
    pub fn start(&mut self) {
        if self.controll.get_status() != Status::Aborted {
            panic!("already started");
        }
        self.controll.set_status(Status::Executing);

        let game_model = self.game_model.clone();
        let controll = self.controll.clone();
        let (cq,user_changes) = channel();
        self.change_queue = cq;

        self.controll.execute_async(move ||{
            let mut cont = controll.clone();
            println!("running");
            match user_changes.recv(){
                Ok(Change::StopThread) => {
                    cont.stop();
                    println!("stopping");
                },
                Ok(message) => ModelAccess::write(game_model.clone(), &message),
                _ => {
                    cont.stop();
                    println!("stopping")
                }
            }
        })
    }
    pub fn enqueue(&self, change:Change){
        self.change_queue.send(change);
    }
    pub fn read_model(&self) -> GameModel{
        self.game_model.read().expect("exists").clone()
    }
    fn write(game_model:Arc<RwLock<GameModel>>, change:&Change){
        match change{
            &Change::BodyViewID(address, changeto) => {
                let mut body = game_model.read().expect("it").get_body(&address).clone();
                body.view_id = changeto;
                game_model.write().expect("it").set_body(&address, body);
            }       
            &Change::Time(to) => game_model.write().expect("it").time = to,
            _ => println!("unhandled")
        }
    }
}

use petgraph::graph::NodeIndex;
use model::BodyAddress;
pub enum Change{
    BodyViewID(BodyAddress, Option<NodeIndex<u32>>),
    Time(Duration),
    StopThread
}
