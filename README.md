<div align="center">
  <img src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&weight=600&size=35&pause=1000&color=FF0000&center=true&vCenter=true&width=800&lines=procsnipe+🎯;task+manager+on+steroids;kill+fps-eating+processes;vim+keybindings+%2F+TUI+aesthetic;built+for+windows+gamers" alt="Typing SVG" />
</div>

<div align="center">
  <img src="https://i.pinimg.com/originals/9d/1b/0e/9d1b0e92276C789b.gif" width="100%" height="2px" />
</div>
<br/>

<div align="center">
    <b>procsnipe</b> is a TUI process manager that doesn't suck.<br/>
    kill fps-eating processes, monitor your system like a hacker, and flex on task manager.<br/>
    <b>windows only</b> 🪟 | <b>built for gamers who want their PC to stop being a potato</b> 🥔
</div>
<br/>

<div align="center">
  <img src="https://skillicons.dev/icons?i=rust,windows&theme=dark" />
</div>
<br/>
### 🔥 features that actually matter

- **real-time process monitoring** - see what's eating your CPU/RAM instantly
- **game detection** - automatically highlights game processes (steam, epic, unity, etc.)
- **quick-kill mode** - snipe processes with a single keypress (`d`)
- **vim keybindings** - because mouse is for casuals
- **lightweight AF** - uses less resources than what it monitors
- **portable .exe** - no installation, no admin\*, just run

<sub>\*admin might be needed to kill protected processes, but the app will still work without it</sub>
<br/>
<div align="center">
  <img src="https://i.pinimg.com/originals/9d/1b/0e/9d1b0e92276C789b.gif" width="100%" height="2px" />
</div>

### 🛠️ installation

**option 1: installer** _(recommended for most users)_

```bash
# 1. Download procsnipe-setup.exe from releases
# 2. Run installer
# 3. Optionally enable "Run at startup" for system tray mode
```

**option 2: portable version** _(no installation)_

```bash
# Download procsnipe.exe from releases
# Run directly - works anywhere
```

**option 3: build from source**

```bash
git clone https://github.com/berochitiri/procsnipe
cd procsnipe

# Build portable:
cargo build --release
.\target\release\procsnipe.exe

# Build installer (requires Inno Setup):
.\build_installer.bat
```
<br/>
<div align="center">
  <img src="https://i.pinimg.com/originals/9d/1b/0e/9d1b0e92276C789b.gif" width="100%" height="2px" />
</div>

### 🎯 system tray mode

run in the background with continuous monitoring:

```bash
procsnipe.exe --tray
```

**what it does:**

- monitors all processes in the background
- shows notifications for high CPU usage (>80%)
- system tray icon with right-click menu
- opens full TUI when needed
- uses minimal resources (~5MB RAM)

**auto-start:**

- enable during installation
- runs silently in tray on Windows startup
- always available when you need it
<br/>
### ⌨️ keybindings (vim-style because we're not animals)

| key       | action                           |
| --------- | -------------------------------- |
| `j` / `↓` | navigate down                    |
| `k` / `↑` | navigate up                      |
| `/`       | search/filter processes          |
| `d`       | **kill selected process**        |
| `g`       | toggle game-only view            |
| `s`       | cycle sort (name → cpu → memory) |
| `?`       | show help                        |
| `q`       | quit                             |
| `ESC`     | exit search/help                 |
<br/>

### 🎮 why this exists

task manager is **bloated**. process hacker is **ugly**. i wanted something that:

1. **looks cool** in a terminal
2. **works fast** (no lag, no bullshit)
3. **helps me game** (find what's killing my fps)

so i built it. now you can use it too.
<br/>
<div align="center">
  <img src="https://i.pinimg.com/originals/9d/1b/0e/9d1b0e92276C789b.gif" width="100%" height="2px" />
</div>

### 📦 tech stack

- **rust** - because performance matters
- **ratatui** - for that sick TUI aesthetic
- **sysinfo** - windows process APIs
- **crossterm** - terminal manipulation

### 🎨 features showcase

**color coding:**

- 🟢 green = game processes
- 🔴 red = high CPU usage (>50%)
- 🟡 yellow = medium CPU usage (>20%)
- ⚪ white = normal processes

**views:**

- press `g` to toggle game-only mode (perfect for finding which launcher is running in the background)
- press `s` to sort by CPU/Memory/Name
- press `/` to filter by process name
<br/>
### ⚠️ disclaimer

killing system processes can brick your session. don't be stupid. i'm not responsible if you kill `explorer.exe` and cry about it.

also, some processes might require **admin privileges** to kill. if it doesn't work, run procsnipe as admin.
<br/>
### 🤝 contributing

PRs welcome. keep it clean, keep it fast, keep it edgy.
<br/>
<div align="center">
  <img src="https://user-images.githubusercontent.com/73097560/115834477-dbab4500-a447-11eb-908a-139a6edaec5c.gif" width="100%"/>
</div>

<div align="center">
  <b>made with 💀 by <a href="https://github.com/berochitiri">berochitiri</a></b>
</div>
