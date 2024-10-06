
pub struct PowerplansStruct {
    
}


impl PowerplansStruct {
    
    pub const HIGHEST_POWER:&'static str = "8C5E7FDA-E8BF-4A96-9A85-A6E23A8C635C";
    pub const MIN_POWER: &'static str  = "A1841308-3541-4FAB-BC81-F71556F20B4A";
    pub const BALANCED: &'static str = "381B4222-F694-41F0-9685-FF5BB260DF2E";

}


use tray_icon::menu::MenuId;
#[derive(Clone)]
pub struct TrayIconMenuID{
    pub min_power: MenuId,
    pub max_power: MenuId,
    pub typical_power:MenuId,
    pub auto_mode_toggle: MenuId,
}
impl TrayIconMenuID {
    pub fn new( minpower:MenuId, max_power: MenuId, typical_power: MenuId, auto_mode_toggle: MenuId) -> Self {
        Self {
        min_power : minpower,
        typical_power : typical_power,
        max_power : max_power,
        auto_mode_toggle: auto_mode_toggle
    }
}
  
}