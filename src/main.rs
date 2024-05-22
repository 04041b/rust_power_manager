//#![windows_subsystem = "windows"]

use std::thread;

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{ Scheduler, TimeUnits};
use tray_icon::menu::{MenuEvent, MenuItem};
use uuid::Uuid;
use winapi::shared::guiddef::GUID;
use winapi::um::powersetting::PowerSetActiveScheme;

// Import week days and WeekDay

use tray_icon::TrayIconBuilder;
use tray_icon::TrayIconEvent;
mod rust_power_error;

fn main() {
    let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    // Add some tasks to it
    scheduler.every(30.seconds()).run(|| {println!("{}", get_power_state()); run_set_powerplan()});

    //let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    // Manually run the scheduler in an event loop
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();


    let main_menu = tray_icon::menu::Menu::new();
    main_menu.append(&MenuItem::new("text", true, None)).unwrap();

    let icon_file = match tray_icon::Icon::from_path(std::path::Path::new("icon.ico"),None) {
        Ok(icon) => icon,
        Err(e) => {rust_power_error::craft_error_window_win(e.to_string(), "error"); panic!("{}", e);}
    };



let tray_icon = TrayIconBuilder::new()
    .with_tooltip("rust power manager - tray")
    .with_icon(icon_file)
    .with_menu(Box::new(main_menu))
    .build()
    .unwrap();
    println!("start");

    let thread_handle = scheduler.watch_thread(std::time::Duration::from_millis(100));


    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |event, event_loop| {
        // We add delay of 16 ms (60fps) to event_loop to reduce cpu load.
        // This can be removed to allow ControlFlow::Poll to poll on each cpu cycle
        // Alternatively, you can set ControlFlow::Wait or use TrayIconEvent::set_event_handler,
        // see https://github.com/tauri-apps/tray-icon/issues/83#issuecomment-1697773065


        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(16)
            
        ));

        if let winit::event::Event::NewEvents(winit::event::StartCause::Init) = event {

            // We create the icon once the event loop is actually running
            // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
            Some(&tray_icon);
            // We have to request a redraw here to have the icon actually show up.
            // Winit only exposes a redraw method on the Window so we use core-foundation directly.

        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
        
    }).unwrap();


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


fn uuid_from_str(uuidstr: &str) -> GUID {
    let uuid_obj:Uuid = uuidstr.parse().unwrap();
    let mut win_guidobj: GUID = Default::default();
    win_guidobj.Data1 = uuid_obj.as_fields().0;
    win_guidobj.Data2 = uuid_obj.as_fields().1;
    win_guidobj.Data3 = uuid_obj.as_fields().2;
    win_guidobj.Data4 = *uuid_obj.as_fields().3;
    return win_guidobj;
}
