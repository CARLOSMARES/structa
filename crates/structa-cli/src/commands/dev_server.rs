use anyhow::Result;
use chrono::Local;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

pub struct DevServer {
    project_root: PathBuf,
    port: u16,
    hot_reload: bool,
    child: Option<std::process::Child>,
}

impl DevServer {
    pub fn new(project_root: PathBuf, port: u16, hot_reload: bool) -> Self {
        Self {
            project_root,
            port,
            hot_reload,
            child: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        print_banner(self.port);

        let main_file = self.project_root.join("src").join("main.structa");
        if !main_file.exists() {
            log_error("main.structa not found at src/main.structa");
            log_info("Run 'structa init' to create a new project");
            return Ok(());
        }

        log_info("Starting development server with Node.js...");
        self.start_server()?;

        if self.hot_reload {
            log_info("Hot reload enabled...");
            self.watch_for_changes()?;
        } else if let Some(ref mut child) = self.child {
            let _ = child.wait();
        }
        Ok(())
    }

    fn start_server(&mut self) -> Result<()> {
        let main_file = self.project_root.join("src").join("main.structa");
        log_info(&format!("Starting: npx tsx {}", main_file.display()));

        let child = Command::new("npx")
            .args(["tsx", main_file.to_str().unwrap_or("src/main.structa")])
            .env("PORT", self.port.to_string())
            .env("HOST", "0.0.0.0")
            .current_dir(&self.project_root)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        self.child = Some(child);
        Ok(())
    }

    fn restart_server(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        log_info("Restarting server...");
        self.start_server()
    }

    fn watch_for_changes(&mut self) -> Result<()> {
        let src_dir = self.project_root.join("src");
        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default().with_poll_interval(Duration::from_millis(100)),
        )?;
        watcher.watch(&src_dir, RecursiveMode::Recursive)?;
        let (reload_tx, reload_rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut timer: Option<std::thread::JoinHandle<()>> = None;
            for event in rx {
                if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                    if let Some(h) = timer.take() {
                        let _ = h.join();
                    }
                    let tx = reload_tx.clone();
                    timer = Some(std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(200));
                        let _ = tx.send(());
                    }));
                }
            }
        });
        loop {
            if reload_rx.recv_timeout(Duration::from_secs(1)).is_ok() {
                log_info("File changed, restarting...");
                if let Err(e) = self.restart_server() {
                    log_error(&format!("Restart failed: {}", e));
                } else {
                    log_success("Server restarted successfully");
                }
            }
        }
    }
}

fn print_banner(port: u16) {
    println!();
    println!("\x1b[32m╔══════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[32m║\x1b[0m \x1b[32m  Structa Development Server                \x1b[0m\x1b[32m║\x1b[0m");
    println!("\x1b[32m╠══════════════════════════════════════════════╣\x1b[0m");
    println!("\x1b[32m║\x1b[0m \x1b[36mPort:\x1b[0m     \x1b[33m{}\x1b[0m                             \x1b[32m║\x1b[0m", port);
    println!("\x1b[32m║\x1b[0m \x1b[36mEngine:\x1b[0m   \x1b[32mNode.js + TSX\x1b[0m                  \x1b[32m║\x1b[0m");
    println!("\x1b[32m╚══════════════════════════════════════════════╝\x1b[0m");
    println!();
}

fn log_info(msg: &str) {
    let t = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\x1b[32m[{t}]\x1b[0m \x1b[36mINFO\x1b[0m     \x1b[32m→\x1b[0m {}",
        msg
    );
}
fn log_warn(msg: &str) {
    let t = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\x1b[32m[{t}]\x1b[0m \x1b[33mWARN\x1b[0m     \x1b[32m→\x1b[0m {}",
        msg
    );
}
fn log_error(msg: &str) {
    let t = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\x1b[32m[{t}]\x1b[0m \x1b[31mERROR\x1b[0m    \x1b[32m→\x1b[0m {}",
        msg
    );
}
fn log_success(msg: &str) {
    let t = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\x1b[32m[{t}]\x1b[0m \x1b[32mOK\x1b[0m      \x1b[32m→\x1b[0m {}",
        msg
    );
}
