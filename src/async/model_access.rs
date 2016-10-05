// This file contains a structure that is responsable for updating the model.
// this is done on a dedicated thread that blocks untill changes enter it.
// all model writes should go trough this structure, each model should have
// a respective singular writer.

// The main idea behind this is to prevent race conditions and group certain
// updates together to prevent readers from getting weird states, such as for
// example a time update should also include a resource tick. So that you never
// get increased resources without increased time.
// It also should prevent cheating by doing model checks, such as, "is this
// change even allowed". Because the clients may have been working on an older
// state where things still could've worked.

use std::thread;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use time::Duration;

use model::{carrying_capacity_earth, GameModel};
use async::thread_status::{ThreadControll, Status};
use std::sync::mpsc::{channel, Sender};

#[derive(Clone)]
pub struct ModelAccess{
    controll:ThreadControll,
    pub game_model:Arc<RwLock<GameModel>>,
    pub change_queue:Sender<Change>
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
        let (cq,user_changes) = channel();
        self.change_queue = cq;

        self.controll.execute_async(move ||{
            match user_changes.recv(){
                Ok(message) => ModelAccess::write(game_model.clone(), &message),
                _ => {
                    // it means that all senders are de-allocated
                    // therefore this thread became useless and the easiest way
                    // of dealing with this is crashing it.
                    panic!("otherside hung up");
                }
            }
        })
    }
    pub fn stop(&mut self){
        let sender = self.change_queue.clone();
        // prevent a deadlock by flushing the thread with messages
        thread::spawn(move ||{
            while let Ok(_) = sender.send(Change::Nothing) {
                thread::yield_now();
            }
            // otherside hung up, this is what we wanted, this thread can die now
        });
        self.controll.stop();
        println!("stopped access");
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
            &Change::Time(increase) => ModelAccess::resource_tick(game_model.write().expect("it"), increase),
            &Change::Nothing => {}
        }
    }
    fn resource_tick(mut game_model:RwLockWriteGuard<GameModel>, increase:Duration){
        game_model.time = game_model.time + increase;
        for body in game_model.galaxy.iter_mut().flat_map(|x| x.bodies.iter_mut().filter(|y| y.population.is_some())){
            body.population = if let Some(mut population) = body.population.clone(){
                population.head_count += population.calc_head_increase(
                    (body.surface_area*carrying_capacity_earth) as i64,
                    increase
                );
                Some(population)
            }else{None}
        }
    }
}

use petgraph::graph::NodeIndex;
use model::BodyAddress;
#[derive(Debug)]
pub enum Change{
    BodyViewID(BodyAddress, Option<NodeIndex<u32>>),
    Time(Duration),
    Nothing // usefull for dealing with controll changes (ie thread abort)
}
