use anyhow::Result;
use chrono::Local;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

use structa_compiler::{compile, Lexer, Parser};
use structa_linker::generate_runtime;

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

        log_info("Compiling .structa files...");
        match self.compile_all() {
            Ok(js) => {
                log_success(&format!("Compiled {} file(s)", js.len()));
                self.start_server(&js)?;
            }
            Err(e) => {
                log_error(&format!("Compilation failed: {}", e));
                return Ok(());
            }
        }

        if self.hot_reload {
            log_info("Hot reload enabled...");
            self.watch_for_changes()?;
        } else if let Some(ref mut child) = self.child {
            let _ = child.wait();
        }
        Ok(())
    }

    fn compile_all(&self) -> Result<Vec<(String, String)>> {
        let src_dir = self.project_root.join("src");
        let mut compiled = Vec::new();

        for entry in walkdir(&src_dir) {
            if let Some(path) = entry {
                let p = std::path::Path::new(&path);
                if p.extension().map_or(false, |ext| ext == "structa") {
                    match std::fs::read_to_string(&path) {
                        Ok(source) => {
                            let mut lexer = Lexer::new(&source);
                            let tokens = lexer.tokenize();
                            let mut parser = Parser::new(tokens);
                            let prog = parser.parse();
                            let js = compile(&prog);

                            let relative = p
                                .strip_prefix(&self.project_root)
                                .unwrap_or(p)
                                .to_string_lossy()
                                .replace('\\', "/");

                            compiled.push((relative, js));
                            log_success(&format!(
                                "Compiled: {}",
                                p.file_name().unwrap_or_default().to_string_lossy()
                            ));
                        }
                        Err(e) => log_error(&format!("Failed to read {}: {}", path, e)),
                    }
                }
            }
        }

        if compiled.is_empty() {
            log_warn("No .structa files found");
        }

        let main_compiled: Vec<_> = compiled
            .iter()
            .filter(|(name, _)| name.contains("main.structa"))
            .cloned()
            .collect();

        if main_compiled.is_empty() {
            log_error("No main.structa found. Run 'structa init' to create a new project.");
            return Ok(Vec::new());
        }

        Ok(main_compiled)
    }

    fn start_server(&mut self, compiled: &[(String, String)]) -> Result<()> {
        let runtime = generate_runtime();

        let mut all_js = String::new();
        all_js.push_str(&runtime);
        all_js.push_str("\n\n// Compiled files\n");

        for (filename, js) in compiled {
            all_js.push_str(&format!("\n// === {} ===\n", filename));
            let js_clean = js
                .lines()
                .filter(|line| !line.trim().starts_with("import { server } from"))
                .collect::<Vec<_>>()
                .join("\n");
            all_js.push_str(&js_clean);
        }

        log_info("Starting server with Node.js...");

        let child = Command::new("node")
            .arg("-e")
            .arg(&all_js)
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

        log_info("Recompiling...");
        match self.compile_all() {
            Ok(js) => {
                log_success(&format!("Compiled {} file(s)", js.len()));
                self.start_server(&js)?;
            }
            Err(e) => {
                log_error(&format!("Compilation failed: {}", e));
            }
        }
        Ok(())
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

fn walkdir(dir: &std::path::Path) -> Vec<Option<String>> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir(&path));
            } else {
                files.push(Some(path.to_string_lossy().to_string()));
            }
        }
    }
    files
}

fn print_banner(port: u16) {
    println!();
    println!("\x1b[32m╔══════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[32m║\x1b[0m \x1b[32m  Structa Development Server                \x1b[0m\x1b[32m║\x1b[0m");
    println!("\x1b[32m╠══════════════════════════════════════════════╣\x1b[0m");
    println!("\x1b[32m║\x1b[0m \x1b[36mPort:\x1b[0m     \x1b[33m{}\x1b[0m                             \x1b[32m║\x1b[0m", port);
    println!("\x1b[32m║\x1b[0m \x1b[36mEngine:\x1b[0m   \x1b[32mRust Compiler\x1b[0m                  \x1b[32m║\x1b[0m");
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
        "\x1b[32m[{t}]\x1b[0m \x1b[33mWARN\x1b[0m     \x1b[32m→\x1b[0m \x1b[33m{}\x1b[0m",
        msg
    );
}

fn log_error(msg: &str) {
    let t = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\x1b[32m[{t}]\x1b[0m \x1b[31mERROR\x1b[0m    \x1b[32m→\x1b[0m \x1b[31m{}\x1b[0m",
        msg
    );
}

fn log_success(msg: &str) {
    let t = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\x1b[32m[{t}]\x1b[0m \x1b[32mOK\x1b[0m      \x1b[32m→\x1b[0m \x1b[32m{}\x1b[0m",
        msg
    );
}
