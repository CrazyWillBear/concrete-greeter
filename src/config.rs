// This is the configuration file. Settings are predetermined as recommended by
// myself, but are in no way required to stay that way. Battery info is disabled
// by default.

// system info
pub const DISPLAY_SYSINFO: bool = true; // display system info

pub const SYS_NAME: bool = false;
pub const KERNEL_VER: bool = true;
pub const OS_VER: bool = false; // buggy - disable if not working
pub const HOST_NAME: bool = true;
pub const CPU_USAGE: bool = true; // uses CPU[0] for benchmark
pub const MEM_USAGE: bool = true;
pub const STROAGE_AVAILABLE: bool = true; // uses disk[0] for benchmark



// battery info
pub const DISPLAY_BATTERY: bool = false; // display battery info

pub const BATTERY_CHARGE_FILE_LOC: &str = "/sys/class/power_supply/BAT0/capacity"; // battery percent file path, not necessary if disabled display battery
pub const MED_BATTERY_THRESHOLD: i32 = 60; // battery symbol turns yellow at this percentage
pub const LOW_BATTERY_THRESHOLD: i32 = 10; // battery symbol turns red at this percentage



// date and time
pub const DATE_TIME: &str = "%m/%d/%y - %I:%M:%S%p";

// Guide here for time display:
// %D – Display date as mm/dd/yy
// %Y – Long year (e.g., 2020)
// %y – Short year (e.g., 20)
// %m – Month (01-12)
// %B – Long month name (e.g., November)
// %b – Short month name (e.g., Nov)
// %d – Day of month (e.g., 01)
// %j – Day of year (001-366)
// %u – Day of week (1-7)
// %A – Full weekday name (e.g., Friday)
// %a – Short weekday name (e.g., Fri)
// %H – Hour (00-23)
// %I – Hour (01-12)
// %M – Minute (00-59)
// %S – Second (00-60)
// %p - AM/PM



// custom message
pub const DISPLAY_CUSTOM_MSG: bool = true; // display a custom message

pub const CUSTOM_MSG: &str = "Welcome!"; // your custom message



// double breakline at beginning and end
pub const DOUBLE_BR: bool = true; // adds an extra breakline at beginning and end of program