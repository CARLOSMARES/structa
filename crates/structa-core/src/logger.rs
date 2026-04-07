use chrono::Local;
use std::sync::Mutex;

static LOGGER: Logger = Logger {
    enabled: Mutex::new(true),
    verbose: Mutex::new(false),
};

pub struct Logger {
    enabled: Mutex<bool>,
    verbose: Mutex<bool>,
}

impl Logger {
    pub fn init(verbose: bool) {
        *LOGGER.verbose.lock().unwrap() = verbose;
        println!("\x1b[32m╔══════════════════════════════════════════════════════════╗\x1b[0m");
        println!("\x1b[32m║\x1b[0m \x1b[32m██████╗ ███████╗███████╗██╗███╗   ██╗██╗   ██╗███████╗\x1b[0m   \x1b[32m║");
        println!("\x1b[32m║\x1b[0m \x1b[32m██╔══██╗██╔════╝██╔════╝██║████╗  ██║██║   ██║██╔════╝\x1b[0m   \x1b[32m║");
        println!("\x1b[32m║\x1b[0m \x1b[32m██████╔╝█████╗  █████╗  ██║██╔██╗ ██║██║   ██║███████╗\x1b[0m   \x1b[32m║");
        println!("\x1b[32m║\x1b[0m \x1b[32m██╔══██╗██╔══╝  ██╔══╝  ██║██║╚██╗██║██║   ██║╚════██║\x1b[0m   \x1b[32m║");
        println!("\x1b[32m║\x1b[0m \x1b[32m██║  ██║███████╗███████╗██║██║ ╚████║╚██████╔╝███████║\x1b[0m   \x1b[32m║");
        println!("\x1b[32m║\x1b[0m \x1b[32m╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝\x1b[0m   \x1b[32m║");
        println!("\x1b[32m╠══════════════════════════════════════════════════════════╣\x1b[0m");
        println!("\x1b[32m║\x1b[0m \x1b[32mStructa Framework v{} - TypeScript-like API Framework\x1b[0m   \x1b[32m║", env!("CARGO_PKG_VERSION"));
        println!("\x1b[32m╚══════════════════════════════════════════════════════════╝\x1b[0m");
        println!();
    }

    pub fn info(&self, msg: &str) {
        if *self.enabled.lock().unwrap() {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "\x1b[32m[{timestamp}]\x1b[0m \x1b[36mINFO\x1b[0m     \x1b[32m→\x1b[0m {}",
                msg
            );
        }
    }

    pub fn warn(&self, msg: &str) {
        if *self.enabled.lock().unwrap() {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!("\x1b[32m[{timestamp}]\x1b[0m \x1b[33mWARN\x1b[0m     \x1b[32m→\x1b[0m \x1b[33m{}\x1b[0m", msg);
        }
    }

    pub fn error(&self, msg: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        println!("\x1b[32m[{timestamp}]\x1b[0m \x1b[31mERROR\x1b[0m    \x1b[32m→\x1b[0m \x1b[31m{}\x1b[0m", msg);
    }

    pub fn debug(&self, msg: &str) {
        if *self.verbose.lock().unwrap() {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!("\x1b[32m[{timestamp}]\x1b[0m \x1b[90mDEBUG\x1b[0m    \x1b[32m→\x1b[0m \x1b[90m{}\x1b[0m", msg);
        }
    }

    pub fn success(&self, msg: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        println!("\x1b[32m[{timestamp}]\x1b[0m \x1b[32mOK\x1b[0m      \x1b[32m→\x1b[0m \x1b[32m{}\x1b[0m", msg);
    }

    pub fn matrix_drop(&self, count: usize) {
        let chars = "ｦｱｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ0123456789";
        for _ in 0..count {
            let c = chars.chars().nth(rand_index(chars.len())).unwrap_or('0');
            print!("\x1b[32m{}\x1b[0m", c);
        }
        println!();
    }
}

fn rand_index(max: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;
    nanos % max
}

pub fn info(msg: &str) {
    LOGGER.info(msg);
}
pub fn warn(msg: &str) {
    LOGGER.warn(msg);
}
pub fn error(msg: &str) {
    LOGGER.error(msg);
}
pub fn debug(msg: &str) {
    LOGGER.debug(msg);
}
pub fn success(msg: &str) {
    LOGGER.success(msg);
}
