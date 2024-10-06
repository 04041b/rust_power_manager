use clokwerk::Scheduler;
use clokwerk::{ Scheduler, TimeUnits};
pub struct auto_mode{
    scheduler: clokwerk::Scheduler,
    thread: clokwerk::ScheduleHandle
}


impl auto_mode{
    pub fn new()->Self{
       let scheduler= Scheduler::new();
        scheduler.every(30.seconds()).run(|| {println!("{}", get_power_state()); auto_run_set_powerplan()});

        auto_mode{
            scheduler: scheduler,
            thread: 
        }
        
    }
    pub fn test(&self){
    }


}