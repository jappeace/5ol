// this is a thread abstraction to provide similar controlls for each thread
// aborted will resund in the thread dying
// executing will call the thread logic every pace_ms
// paused will halt execution of thread logic and put itself in a sleep cycle
// untill a state change

use std::time;
use std::sync::{Arc, RwLock};
use std::thread;

fn sleep(){
    let one_ms = time::Duration::from_millis(1);
    thread::yield_now();
    // just yielding is to little, the thread will still consume 1 core
    thread::sleep(one_ms); 
}

#[derive(Clone,Copy, PartialEq)]
pub enum Status{
    Aborted,
    Paused,
    Executing
}

// allow the pacing of the game trough this struct
pub struct ThreadStatus{
    pub pace_ms:u64,
    pub status:Status
}
impl ThreadStatus{
    fn new() -> ThreadStatus{
        ThreadStatus{
            pace_ms:0,
            status:Status::Executing,
        }
    }
}
#[derive(Clone)]
pub struct ThreadControll{
    pub status:Arc<RwLock<ThreadStatus>>,
}
impl ThreadControll{
    pub fn new()->ThreadControll{
        ThreadControll{
            status:Arc::new(RwLock::new(ThreadStatus::new()))
        }
    }
    pub fn execute_async<F, T>(&self, threadlogic:F) -> ()
        where F: Fn() -> T, F: Send + 'static, T: Send + 'static
    {
        self.status.write().expect("running").status = Status::Executing;
        let controll = self.status.clone();
        thread::spawn(move|| {
            with_default_controlls(controll, threadlogic); 
        });
    }
    pub fn toggle_pause(&mut self){
        let newstatus = match self.status.read().expect("poisen").status {
            Status::Paused => Status::Executing,
            Status::Executing => Status::Paused,
            x => x
        };
        self.set_status(newstatus);
    }
    pub fn set_status(&mut self, status:Status){
        self.status.write().expect("poisen").status = status;
    }
    pub fn get_status(&self) -> Status{
        self.status.read().expect("reading status").status
    }
    pub fn set_pace(&mut self, pace_ms:u64){
        self.status.write().expect("poisen").pace_ms = pace_ms;
    }
    pub fn stop(&mut self){
        self.set_status(Status::Aborted);
    }
}

fn with_default_controlls<F, T>(controll:Arc<RwLock<ThreadStatus>>, threadlogic:F) -> ()
    where F: Fn() -> T, F: Send + 'static, T: Send + 'static {
    let no_poisen = "no poisen";
    loop{
        let pace = controll.read().expect(no_poisen).pace_ms;
        let pace_duration = time::Duration::from_millis(pace);
        let status = controll.read().expect(no_poisen).status;
        match status {
            Status::Aborted => break,
            Status::Paused =>{
                sleep();
                continue;
            }
            Status::Executing => {
                threadlogic();
                if pace > 0{
                    thread::sleep(pace_duration);
                }
            }
        }
    }
}

