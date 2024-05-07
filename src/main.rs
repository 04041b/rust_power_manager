// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{ Scheduler, TimeUnits};
use uuid::Uuid;
use winapi::shared::guiddef::GUID;
use winapi::um::powersetting::PowerSetActiveScheme;
// Import week days and WeekDay
use std::thread;
use std::time::Duration;
fn main() {
    let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    // Add some tasks to it
    // problem with print https://stackoverflow.com/questions/76630819/print-doesnt-print-the-message-when-used-inside-a-loop-in-rust
    scheduler.every(30.seconds()).run(|| {println!("{}", get_power_state()); run_set_powerplan()});

    // Manually run the scheduler in an event loop
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}


fn run_set_powerplan() {
    let powerplan;
    match get_power_state() {
        1 =>   powerplan = "8C5E7FDA-E8BF-4A96-9A85-A6E23A8C635C",
        0=> powerplan = "381B4222-F694-41F0-9685-FF5BB260DF2E",
        _ => powerplan = "381B4222-F694-41F0-9685-FF5BB260DF2E"
    }
    let u16powerplanform = uuid_from_str(&powerplan);
    unsafe{
        PowerSetActiveScheme(std::ptr::null_mut(),&u16powerplanform );
    }
    println!("new powerset")

}

fn get_power_state()->u8  {

    let mut powerbase: winapi::um::winbase::SYSTEM_POWER_STATUS = Default::default();

    unsafe { 
        winapi::um::winbase::GetSystemPowerStatus(&mut powerbase);
    }
    return powerbase.ACLineStatus;

}

//I hate microsoft
// fn uuid_str_to_uuid(strguid: &str) -> GUID {
//     println!("{}", strguid.split("-").next().unwrap());
//     let Data1= hex::decode(strguid.split("-").next().unwrap()).unwrap();
//     let Data2 = hex::decode(strguid.split("-").next().unwrap()).unwrap();
//     let Data3 = hex::decode(strguid.split("-").next().unwrap()).unwrap();
//     let Data4 = hex::decode(strguid.split("-").next().unwrap()).unwrap();


//     return GUID {Data1:293829,Data2:23290,Data3:23290,Data4:[1,2,3,4,5,6,7,8]};
// } 

fn uuid_from_str(uuidstr: &str) -> GUID {
    let uuid_obj:Uuid = uuidstr.parse().unwrap();
    let mut win_guidobj: GUID = Default::default();
    win_guidobj.Data1 = uuid_obj.as_fields().0;
    win_guidobj.Data2 = uuid_obj.as_fields().1;
    win_guidobj.Data3 = uuid_obj.as_fields().2;
    win_guidobj.Data4 = *uuid_obj.as_fields().3;
    return win_guidobj;
}