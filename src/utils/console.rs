use console::style;
use crate::cli::isDebug;
use crate::LOG_PATH;
use chrono::Local;
use crate::utils::util::writeLogFile;

pub enum ConsoleType {
    Info,
    Success,
    Warning,
    Err,
}

pub fn writeConsole(consoleType: ConsoleType, message: &str) {
    let title = match consoleType {
        ConsoleType::Info => style("Info   ").cyan(),
        ConsoleType::Success => style("Success").green(),
        ConsoleType::Warning => style("Warning").yellow(),
        ConsoleType::Err => style("Error  ").red().on_black().bold()
    };
    println!("  {}      {}", title, message);
    if isDebug() {
        let time = Local::now().format("%T").to_string();
        // let tieme = SystemTime::now().duration_since(UNIX_EPOCH);
        writeLogFile(&*LOG_PATH, &format!("{} DriverIndexer-{}", time, message));
    }
}