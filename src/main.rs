use colored::Colorize;
use libmacchina::{GeneralReadout, KernelReadout};

fn main() {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();

    use libmacchina::traits::KernelReadout as _;
    let kernel_readout = KernelReadout::new();

    let name = general_readout.username().unwrap().to_lowercase().replace(" ", "");
    let os = kernel_readout.os_type().unwrap().to_lowercase().replace(" ", "");
    let uptime = format_uptime(general_readout.uptime().unwrap());

    let (art1, art2) = match os.to_lowercase().as_str() {
        s if s.contains("windows") => ("\\\\//", "//\\\\"), // windows (windows)
        s if s.contains("darwin") => ("/\\\\", "\\\\/"), // darwin (macos)
        _ => (" /\\ ", "/  \\"), // linux (arch)
    };


    println!(" {}  {}", art1.bright_blue(), uptime.bright_black());
    println!(" {}  {}{}{}", art2.bright_blue(), name.bright_yellow(), "@".bright_red(), os.bright_blue());
}

fn format_uptime(seconds: usize) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;

    let day_str = if days == 1 { "day" } else { "days" };
    let hour_str = if hours == 1 { "hour" } else { "hours" };

    format!("{} {}, {} {}", days, day_str, hours, hour_str)
}