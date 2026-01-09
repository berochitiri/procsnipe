use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::{ProcessesToUpdate, System};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};

pub struct TrayApp {
    sys: Arc<Mutex<System>>,
    last_notification: Instant,
    notification_cooldown: Duration,
}

impl TrayApp {
    pub fn new() -> Self {
        Self {
            sys: Arc::new(Mutex::new(System::new_all())),
            last_notification: Instant::now(),
            notification_cooldown: Duration::from_secs(60), // Don't spam notifications
        }
    }

    pub fn run(&mut self) -> Result<()> {
        println!("🎯 procsnipe running in system tray");
        println!("⚠️  DISCLAIMER: procsnipe monitors system processes in the background.");
        println!("   This is for performance monitoring only.");
        println!("   Killing critical processes can crash your system.");
        println!("   Use at your own risk.\n");

        // Create tray icon
        let tray_menu = Menu::new();

        let open_item = MenuItem::new("Open procsnipe", true, None);
        let about_item = MenuItem::new("About", true, None);
        let quit_item = MenuItem::new("Exit", true, None);

        tray_menu.append(&open_item)?;
        tray_menu.append(&about_item)?;
        tray_menu.append(&quit_item)?;

        // Load icon
        let icon_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("assets")
            .join("icon.png");

        let icon_data = if icon_path.exists() {
            image::open(&icon_path).ok().and_then(|img| {
                let rgba = img.to_rgba8();
                Icon::from_rgba(rgba.to_vec(), rgba.width(), rgba.height()).ok()
            })
        } else {
            None
        };

        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("procsnipe - Process Monitor")
            .with_icon(icon_data.unwrap_or_else(|| {
                // Fallback: create a simple red icon programmatically
                let size = 32;
                let mut rgba = vec![0u8; (size * size * 4) as usize];
                for i in 0..size * size {
                    let idx = (i * 4) as usize;
                    rgba[idx] = 255; // R
                    rgba[idx + 1] = 0; // G
                    rgba[idx + 2] = 0; // B
                    rgba[idx + 3] = 255; // A
                }
                Icon::from_rgba(rgba, size, size).unwrap()
            }))
            .build()?;

        // Event loop
        let menu_channel = MenuEvent::receiver();

        loop {
            // Check for menu events
            if let Ok(event) = menu_channel.try_recv() {
                if event.id == open_item.id() {
                    // Launch TUI in new process
                    self.launch_tui()?;
                } else if event.id == about_item.id() {
                    self.show_about();
                } else if event.id == quit_item.id() {
                    println!("Exiting procsnipe tray...");
                    break;
                }
            }

            // Monitor processes
            self.monitor_processes()?;

            // Sleep to avoid high CPU usage
            std::thread::sleep(Duration::from_secs(5));
        }

        Ok(())
    }

    fn monitor_processes(&mut self) -> Result<()> {
        let mut sys = self.sys.lock().unwrap();
        sys.refresh_processes(ProcessesToUpdate::All, true);

        // Check for high CPU usage
        let mut high_cpu_processes = Vec::new();
        let mut game_detected = false;

        for (_, process) in sys.processes() {
            let cpu = process.cpu_usage();

            if cpu > 80.0 {
                high_cpu_processes.push((process.name().to_string_lossy().to_string(), cpu));
            }

            // Check for games
            let name = process.name().to_string_lossy().to_lowercase();
            if self.is_game_process(&name) && !game_detected {
                game_detected = true;
            }
        }

        // Send notifications (with cooldown)
        if self.last_notification.elapsed() > self.notification_cooldown {
            if !high_cpu_processes.is_empty() {
                let top_process = &high_cpu_processes[0];
                println!("⚠️  High CPU: {} ({:.1}%)", top_process.0, top_process.1);
                self.last_notification = Instant::now();
            }
        }

        Ok(())
    }

    fn is_game_process(&self, name: &str) -> bool {
        let indicators = [
            "game", "steam", "epic", "uplay", "origin", "riot", "valorant", "league", "csgo",
            "cs2", "dota",
        ];
        indicators.iter().any(|&i| name.contains(i))
    }

    fn launch_tui(&self) -> Result<()> {
        use std::process::Command;

        let exe_path = std::env::current_exe()?;

        #[cfg(target_os = "windows")]
        {
            // Launch in new console window
            Command::new("cmd")
                .args(&["/C", "start", "procsnipe", exe_path.to_str().unwrap()])
                .spawn()?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            Command::new(&exe_path).spawn()?;
        }

        Ok(())
    }

    fn show_about(&self) {
        println!("\n╔════════════════════════════════════╗");
        println!("║        procsnipe v1.0              ║");
        println!("║   TUI Process Manager for Windows  ║");
        println!("╟────────────────────────────────────╢");
        println!("║  ⚠️  DISCLAIMER:                   ║");
        println!("║  - Monitors system processes       ║");
        println!("║  - Can terminate processes         ║");
        println!("║  - Use at your own risk            ║");
        println!("║  - Killing system processes can    ║");
        println!("║    cause system instability        ║");
        println!("╟────────────────────────────────────╢");
        println!("║  Made with 💀 by berochitiri       ║");
        println!("╚════════════════════════════════════╝\n");
    }
}
