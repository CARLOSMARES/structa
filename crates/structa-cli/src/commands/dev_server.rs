use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

const RELOAD_DEBOUNCE_MS: u64 = 300;

pub struct DevServer {
    project_root: PathBuf,
    port: u16,
    hot_reload: bool,
    child: Option<Child>,
    watcher: Option<RecommendedWatcher>,
    stop_tx: Option<Sender<()>>,
}

impl DevServer {
    pub fn new(project_root: PathBuf, port: u16, hot_reload: bool) -> Self {
        Self {
            project_root,
            port,
            hot_reload,
            child: None,
            watcher: None,
            stop_tx: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        print_banner(self.port);
        
        if !self.check_dependencies() {
            return Ok(());
        }

        println!("\n📦 Building .structa files...");
        if !self.compile_all() {
            println!("\n❌ Build failed. Fix errors and restart.");
            return Ok(());
        }

        println!("\n🔄 Starting server...\n");
        
        self.spawn_server()?;
        
        if self.hot_reload {
            self.start_file_watcher()?;
        }

        self.wait_for_shutdown().await;
        
        Ok(())
    }

    fn check_dependencies(&self) -> bool {
        let package_json = self.project_root.join("package.json");
        
        if !package_json.exists() {
            println!("❌ No project found. Run 'structa init' first.");
            return false;
        }

        let node_check = Command::new("node")
            .arg("--version")
            .output();

        match node_check {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("✅ Node.js {}", version.trim());
                true
            }
            _ => {
                println!("❌ Node.js not found. Please install Node.js.");
                false
            }
        }
    }

    fn compile_all(&self) -> bool {
        let src_dir = self.project_root.join("src");
        if !src_dir.exists() {
            println!("⚠️  No src/ directory found.");
            return true;
        }

        let dist_dir = self.project_root.join("dist");
        std::fs::create_dir_all(&dist_dir).ok();

        let mut all_success = true;
        let mut file_count = 0;

        fn walk_and_compile(
            src_dir: &Path,
            dist_dir: &Path,
            all_success: &mut bool,
            file_count: &mut usize,
        ) {
            if let Ok(entries) = std::fs::read_dir(src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let relative = path.strip_prefix(src_dir).unwrap_or(&path);
                        let target = dist_dir.join(relative);
                        std::fs::create_dir_all(&target).ok();
                        walk_and_compile(&path, &target, all_success, file_count);
                    } else if path.extension().map_or(false, |e| e == "structa") {
                        *file_count += 1;
                        if !compile_file(&path, dist_dir) {
                            *all_success = false;
                        }
                    } else if path.extension().map_or(false, |e| e == "ts") {
                        let relative = path.strip_prefix(src_dir).unwrap_or(&path);
                        let ts_output = dist_dir.join(relative.with_extension("js"));
                        if let Some(parent) = ts_output.parent() {
                            std::fs::create_dir_all(parent).ok();
                        }
                        let _ = std::fs::copy(&path, &ts_output).ok();
                    }
                }
            }
        }

        walk_and_compile(&src_dir, &dist_dir, &mut all_success, &mut file_count);

        if file_count == 0 {
            println!("⚠️  No .structa files found.");
        } else {
            println!("✅ Compiled {} .structa file(s)", file_count);
        }

        all_success
    }

    fn spawn_server(&mut self) -> Result<()> {
        let main_js = self.project_root.join("dist").join("src").join("main.js");
        
        if !main_js.exists() {
            println!("❌ dist/main.js not found. Run 'structa build' first.");
            return Ok(());
        }

        let mut cmd = Command::new("node");
        cmd.arg(&main_js)
           .env("PORT", self.port.to_string())
           .env("STRUCTA_MODE", "development")
           .env("STRUCTA_HOT_RELOAD", if self.hot_reload { "1" } else { "0" })
           .current_dir(&self.project_root)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        let pid = child.id();

        println!("✅ Server started (PID: {})", pid);
        println!("🌐 http://localhost:{}\n", self.port);

        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    println!("{}", line);
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().flatten() {
                    eprintln!("{}", colorize_err(&line));
                }
            });
        }

        self.child = Some(child);
        Ok(())
    }

    fn start_file_watcher(&mut self) -> Result<()> {
        let src_dir = self.project_root.join("src");
        let (tx, rx) = channel();
        let (stop_tx, stop_rx) = channel::<()>();
        
        self.stop_tx = Some(stop_tx);

        let mut watcher = RecommendedWatcher::new(
            move |res: std::result::Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default().with_poll_interval(Duration::from_secs(1)),
        )?;

        watcher.watch(&src_dir, RecursiveMode::Recursive)?;
        self.watcher = Some(watcher);

        println!("👀 Watching for file changes...\n");

        let project_root = self.project_root.clone();
        std::thread::spawn(move || {
            Self::watch_loop(rx, stop_rx, project_root);
        });

        Ok(())
    }

    fn watch_loop(rx: Receiver<Event>, stop_rx: Receiver<()>, project_root: PathBuf) {
        let mut last_reload = Instant::now();
        let mut pending_changes: Vec<PathBuf> = Vec::new();

        loop {
            match stop_rx.try_recv() {
                Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }

            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    for path in event.paths {
                        if path.extension().map_or(false, |e| e == "structa") {
                            pending_changes.push(path);
                        }
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    if !pending_changes.is_empty() 
                        && last_reload.elapsed() > Duration::from_millis(RELOAD_DEBOUNCE_MS) 
                    {
                        println!("\n🔄 File changed: {:?}", pending_changes.first().and_then(|p| p.file_name()));
                        pending_changes.clear();
                        last_reload = Instant::now();

                        Self::trigger_reload(&project_root);
                    }
                }
                Err(_) => break,
            }
        }
    }

    fn trigger_reload(project_root: &Path) {
        print!("\n📦 Rebuilding... ");
        
        let src_dir = project_root.join("src");
        let dist_dir = project_root.join("dist");
        
        let mut success = true;
        let mut file_count = 0;

        if let Ok(entries) = std::fs::read_dir(&src_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                if path.extension().map_or(false, |e| e == "structa") {
                    file_count += 1;
                    if !compile_file(&path, &dist_dir) {
                        success = false;
                    }
                }
            }
        }

        if success {
            println!("✅ {} file(s)", file_count);
        } else {
            println!("❌ Failed");
        }
    }

    async fn wait_for_shutdown(&mut self) {
        tokio::signal::ctrl_c().await.ok();
        
        println!("\n👋 Shutting down...");
        
        if let Some(mut child) = self.child.take() {
            child.kill().ok();
            child.wait().ok();
        }
        
        drop(self.stop_tx.take());
        
        println!("✅ Server stopped");
    }
}

fn compile_file(source_path: &Path, dist_dir: &Path) -> bool {
    let src_ancestor = source_path.ancestors().nth(2).unwrap_or(source_path);
    let relative = source_path.strip_prefix(src_ancestor).unwrap_or(source_path);
    
    let target = dist_dir.join(relative.with_extension("js"));
    
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let source = match std::fs::read_to_string(source_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to read {}: {}", source_path.display(), e);
            return false;
        }
    };

    let final_output = convert_structa_to_ts(&source);
    
    match std::fs::write(&target, final_output) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("❌ Failed to write {}: {}", target.display(), e);
            false
        }
    }
}

fn convert_structa_to_ts(source: &str) -> String {
    let mut result = source.to_string();
    
    result = regex::Regex::new(r##"controller\s+(\w+)\s+"([^"]+)""##).unwrap()
        .replace_all(&result, |caps: &regex::Captures| {
            format!("@Controller({}) class {} {{", caps.get(2).unwrap().as_str(), caps.get(1).unwrap().as_str())
        }).to_string();
    
    result = regex::Regex::new(r"service\s+(\w+)").unwrap()
        .replace_all(&result, |caps: &regex::Captures| {
            format!("@Injectable() class {} {{", caps.get(1).unwrap().as_str())
        }).to_string();
    
    result = regex::Regex::new(r"dto\s+(\w+)\s+\{([^}]*)\}").unwrap()
        .replace_all(&result, |caps: &regex::Captures| {
            let name = caps.get(1).unwrap().as_str();
            let fields = caps.get(2).unwrap().as_str();
            let ts_fields: String = fields.lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| {
                    let parts: Vec<&str> = l.trim().split(':').collect();
                    if parts.len() >= 2 {
                        format!("{}: {};", parts[0].trim(), parts[1].trim().trim_end_matches(';'))
                    } else {
                        l.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n  ");
            format!("interface {} {{\n  {}\n}}", name, ts_fields)
        }).to_string();
    
    result = regex::Regex::new(r##"(Get|Post|Put|Delete|Patch)\s+"([^"]+)""##).unwrap()
        .replace_all(&result, |caps: &regex::Captures| {
            format!("@{}('{}')", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())
        }).to_string();
    
    result = regex::Regex::new(r##"from\s+['"]([^'"]+)['"]"##).unwrap()
        .replace_all(&result, |caps: &regex::Captures| {
            let path = caps.get(1).unwrap().as_str();
            if path.starts_with('.') || path.starts_with('@') {
                format!("from '{}'", path)
            } else {
                format!("from '{}'", path)
            }
        }).to_string();
    
    result
}

fn print_banner(port: u16) {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Structa Dev Server                         ║");
    println!("╠══════════════════════════════════════════════╣");
    println!("║  Port:     {}                                 ║", port);
    println!("║  Mode:     Development                       ║");
    println!("╚══════════════════════════════════════════════╝");
}

fn colorize_err(s: &str) -> String {
    format!("⚠️  {}", s)
}
