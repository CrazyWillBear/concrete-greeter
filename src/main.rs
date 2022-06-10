use std::process::Command;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

mod config;
mod colors;

fn main() {

    // double breakline
    if config::DOUBLE_BR { println!(); }

    // indent
    print!("\t");

    // custom message
    if config::DISPLAY_CUSTOM_MSG {
        print!("{}{i}{j}", colors::GOLD, i = config::CUSTOM_MSG, j = colors::RESET);
    }

    // break
    print!("   -   ");

    // date + time
    let output = Command::new("date")
            .arg(format!("+\"{}\"", config::DATE_TIME))
            .output()
            .expect("failed to execute process");
    print!("{}{i}{j}", colors::BOLD_BLUE, i = String::from_utf8(output.stdout).unwrap().replace("\"", ""), j = colors::RESET);

    // system info
    if config::DISPLAY_SYSINFO {
        let mut sys = System::new_all();
        sys.refresh_all();

        print!("{}\t", colors::BOLD_MAGENTA);
        if config::SYS_NAME { print!("Name: {}   -   ", sys.name().unwrap()); }
        if config::KERNEL_VER { print!("Kernel Ver: {}   -   ", sys.kernel_version().unwrap()); }
        if config::OS_VER { print!("OS Ver: {:?}   -   ", sys.os_version()); }
        if config::HOST_NAME { print!("Host: {}   -   ", sys.host_name().unwrap()); }
        if config::CPU_USAGE { print!("CPU: {}% at {i}hz   -   ", sys.cpus()[0].cpu_usage().round(), i = sys.cpus()[0].frequency()); }

        // breakline
        println!();

        // indent and color assignment
        print!("{}\t", colors::BOLD_CYAN);

        if config::MEM_USAGE { print!("Memory: {}mb/{i}mb   -   ", sys.used_memory() / 1000, i = sys.total_memory() / 1000); }
        if config::STROAGE_AVAILABLE { print!("Disk: {}gb/{i}gb", sys.disks()[0].available_space() / 1000000000, i = sys.disks()[0].total_space() / 1000000000); }
    }

    // battery info
    if config::DISPLAY_BATTERY {
        let output = Command::new("/bin/cat")
            .arg(config::BATTERY_CHARGE_FILE_LOC)
            .output()
            .expect("failed to execute process");

        let str_output = String::from_utf8(output.stdout).unwrap().replace("\n", "");
        let int_output = str_output.parse::<i32>().unwrap();
        let full_bars = int_output / 10i32;
        let empty_bars = int_output / 10i32 - 10i32;


        let mut color = colors::GREEN;

        if int_output <= config::MED_BATTERY_THRESHOLD { color = colors::YELLOW; }
        if int_output <= config::LOW_BATTERY_THRESHOLD { color = colors::RED; }

        print!("   -   {}[{i}{j}] {k}%{l}", color, i = (0..full_bars).fold(String::new(), |b, _| b + "#"), j = (0..empty_bars + 2).fold(String::new(), |b, _| b + "-"), k = str_output, l = colors::RESET);
    }
    
    println!();

    // double breakline
    if config::DOUBLE_BR { println!(); }
    
}
