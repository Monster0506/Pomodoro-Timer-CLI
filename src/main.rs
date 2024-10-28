use clap::Parser;
use std::time::Duration;
use tokio::time;
#[tokio::main]
async fn main() {
    let config = PomodoroConfig::parse(); // Parse CLI args
    println!("Pomodoro timer started!");
    start_pomodoro(config).await;
}

#[derive(Parser)]
struct PomodoroConfig {
    #[arg(short, long, default_value_t = 25)]
    work: u64, // Work interval in minutes

    #[arg(short, long, default_value_t = 5)]
    break_time: u64, // Short break interval in minutes

    #[arg(short, long, default_value_t = 15)]
    long_break: u64, // Long break interval in minutes

    #[arg(short, long, default_value_t = 4)]
    cycles: u8, // Number of cycles before a long break

    #[arg(long, default_value_t = false)]
    debug: bool, // Debug flag for testing in seconds instead of minutes
}

async fn run_timer(duration: u64, label: &str, debug: bool) {
    let mut remaining = duration;
    let interval = if debug { 1 } else { 60 };
    while remaining > 0 {
        println!("{}: {} minutes left", label, remaining);
        time::sleep(Duration::from_secs(interval)).await; // Wait for a minute
        remaining -= 1;
    }
    println!("{} session complete!", label);
}

async fn start_pomodoro(config: PomodoroConfig) {
    for cycle in 1..=config.cycles {
        println!("Starting work session {} of {}", cycle, config.cycles);
        run_timer(config.work, "Work", config.debug).await;
        if cycle < config.cycles {
            println!("Starting short break");
            run_timer(config.break_time, "Break", config.debug).await;
        } else {
            println!("Starting long break");
            run_timer(config.long_break, "Long Break", config.debug).await;
        }
    }
    println!("Pomodoro session complete!");
}
