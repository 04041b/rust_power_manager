#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::borrow::Borrow;

use std::cell::{Ref, RefCell};
use std::sync::{Mutex,Arc};
use std::thread;
use lazy_static::lazy_static;

// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{ Scheduler, TimeUnits};
use once_cell::sync::Lazy;
use tray_icon::menu::{MenuEvent, MenuItem};
use uuid::Uuid;
use winapi::shared::guiddef::GUID;
use winapi::um::powersetting::PowerSetActiveScheme;

// Import week days and WeekDay

use tray_icon::{TrayIcon, TrayIconBuilder};
use tray_icon::TrayIconEvent;
use powerplans::TrayIconMenuID;
use powerplans::PowerplansStruct;
mod rust_power_error;
mod powerplans;

static mut tray_icon: Lazy<Mutex<TrayIcon>> = Lazy::new(||Mutex::new(TrayIconBuilder::new().build().unwrap()));

fn main() {
    let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    // Add some tasks to it
    //scheduler.every(30.seconds()).run(|| {println!("{}", get_power_state()); auto_run_set_powerplan()});

    //let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    // Manually run the scheduler in an event loop
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();


    let main_menu = tray_icon::menu::Menu::new();

    let sub_menu_power_set = tray_icon::menu::Submenu::new("set power plan", true);
    let high_power_plan_menu_item = MenuItem::new("high performance plan", true, None);
    let min_power_use_plan_menu_item = MenuItem::new("min power plan", true, None);
    let typical_power_plan_menu_item = MenuItem::new("typical power plan", true, None);
    
    sub_menu_power_set.append(&high_power_plan_menu_item).unwrap();
    sub_menu_power_set.append(&min_power_use_plan_menu_item).unwrap();
    sub_menu_power_set.append(&typical_power_plan_menu_item).unwrap();
    main_menu.append(&sub_menu_power_set).unwrap();


    let auto_mode_switch_menu = MenuItem::new("toggle battery mode", true, None);
    main_menu.append(&auto_mode_switch_menu).unwrap();
    
    
    let powerplans_menu_id = TrayIconMenuID::new(min_power_use_plan_menu_item.into_id(),
    high_power_plan_menu_item.into_id(),
     typical_power_plan_menu_item.into_id(),
    auto_mode_switch_menu.into_id()
   
   );

    let icon_file = match tray_icon::Icon::from_path(std::path::Path::new("icon.ico"),None) {
        Ok(icon) => icon,
        Err(e) => {rust_power_error::craft_error_window_win(e.to_string(), "error"); panic!("{}", e);}
    };


    unsafe {
    *tray_icon.lock().unwrap() = TrayIconBuilder::new()
        .with_tooltip("rust power manager - tray")
        .with_icon(icon_file)
        .with_menu(Box::new(main_menu))
        .build()
        .unwrap();
        println!("start");
            
}

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
            unsafe {
            Some(&tray_icon);}
            // We have to request a redraw here to have the icon actually show up.
            // Winit only exposes a redraw method on the Window so we use core-foundation directly.

        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
        if let Ok(event) = menu_channel.try_recv() {
            println!("{event:?}");
            menu_power_set(event, powerplans_menu_id.borrow());
        }
        
    }).unwrap();


}


fn menu_power_set(e:MenuEvent, powerplan_menu_id: &TrayIconMenuID) {
    let menu_event_id = e.id(); 
    let powerplan;
    if  *menu_event_id == powerplan_menu_id.max_power {
        powerplan =  PowerplansStruct::HIGHEST_POWER;
    }
    else if *menu_event_id == powerplan_menu_id.min_power {
        powerplan=  PowerplansStruct::MIN_POWER;
    }
    else {
        powerplan =  PowerplansStruct::BALANCED;
    }
    set_power_from_string(&powerplan)
    
}

fn auto_run_set_powerplan() {
    let powerplan;
    match get_power_state() {
        1 =>   powerplan = PowerplansStruct::HIGHEST_POWER,
        0=> powerplan = PowerplansStruct::BALANCED,
        _ => powerplan = PowerplansStruct::BALANCED,
    }
    set_power_from_string(&powerplan);
    println!("new powerset")

}

fn set_power_from_string(powerplan_string: &str ) {
    let u16powerplanform = uuid_from_str(&powerplan_string);
    unsafe{
        PowerSetActiveScheme(std::ptr::null_mut(),&u16powerplanform );
    }
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
