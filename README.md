<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Token%20Harbor%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Auto%20Signup%20%7C%20Free%20Model%20Unlock%20%7C%20API%20Key%20%7C%20Chat%20and%20Usage%20Report&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Signup+via+Disposable+Inbox+%7C+Turnstile+Solver;Email+Verification+%7C+Free+Model+Unlock;API+Key+Creation+%7C+Chat+%26+Usage+Report;Enroll+Mode+%7C+Multi-Key+%7C+Proxy+Support"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Token%20Harbor-111111"/>
  <img alt="build" src="https://github.com/Yuurisan-N1/Token-Harbor/actions/workflows/build.yml/badge.svg"/>
  <img alt="proxy" src="https://img.shields.io/badge/Proxy-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Token Harbor Bot</b> is a Rust automation client for Token Harbor.<br/>
  It opens a disposable mailbox, signs up a fresh account through a Turnstile solver, confirms the verification email, unlocks free models, creates an API key, and on every cycle sends one chat to keep the account active and reads the usage headers, with enroll mode to register any number of accounts in one run, all with proxy support.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

<hr/>

<p align="center">
  <img src="image/image.png" alt="preview" width="100%"/>
</p>

<hr/>

---

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Bot](#running-the-bot)
- [Run Modes](#run-modes)
- [Download Prebuilt Binary](#download-prebuilt-binary)
- [Features](#features)
- [File Structure](#file-structure)
- [Continuous Integration](#continuous-integration)
- [Disclaimer](#disclaimer)

---

## Requirements

- Rust stable toolchain (install from [rustup.rs](https://rustup.rs), only needed if building from source)
- Git

---

## Installation

### Install Rust

**Linux / macOS:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:

```bash
rustc --version
```

**Windows:**

Download and run the installer from https://rustup.rs, then reopen the terminal and verify:

```cmd
rustc --version
```

**Termux (Android):**

```bash
pkg update && pkg install proot-distro
proot-distro install ubuntu
proot-distro login ubuntu
```

Then inside Ubuntu:

```bash
apt update && apt install -y curl git build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

### Clone and Build

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Token-Harbor.git
cd Token-Harbor
```

**Build the release binary:**

```bash
./run.sh build
```

---

## Configuration

### 1. API Keys (data.txt)

`data.txt` holds the API keys of enrolled accounts, one per line. On the first run the file can be empty; the bot signs up a fresh account and writes its key automatically. Use enroll mode to fill the file with multiple keys before the first run cycle.

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to connect directly):

```
http://user:pass@host:port
socks5://user:pass@host:port
```

Proxies are paired to accounts by index. Proxy credentials are masked in the log.

### 3. Bot Settings (config.json)

`sleep_seconds` controls how many seconds the bot waits between cycles. The value is clamped between 60 and 86400 seconds. An optional `timezone` key sets the timezone used during signup. The default sleep is 3600 seconds (one hour).

### 4. Solver (solver/solver.yaml)

`solver/solver.yaml` holds the Turnstile solver endpoint used during signup. The default URL is already set:

```yaml
solver:
  url: https://solver.yuurisan.my.id
```

---

## Running the Bot

Make `run.sh` executable first (Linux / macOS / Termux):

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh run            # build if needed and run one cycle in foreground
./run.sh enroll <n>     # sign up n accounts and save every key to data.txt
./run.sh bg             # build if needed and run in background, logs to tokenharbor-bot.log
./run.sh bgprobe        # run probe in background
./run.sh screen         # build if needed and run in a detached screen session
./run.sh stop           # stop the background or screen session
./run.sh probe          # check the site, the solver, and every proxy
./run.sh selftest       # offline checks, no network
./run.sh build          # build only, do not run
```

For background mode, tail the log anytime:

```bash
tail -f tokenharbor-bot.log
```

For screen mode, attach anytime:

```bash
screen -r tokenharbor-bot
```

Press `Ctrl+C` once to stop the bot cleanly when running in foreground.

### Manual run

```bash
# Linux / macOS / Termux
./target/release/tokenharbor-bot

# Windows
.\target\release\tokenharbor-bot.exe
```

---

## Run Modes

| Mode | Command | Description |
|---|---|---|
| run | `./run.sh run` | One cycle: sign up if no key exists, then chat and usage report |
| enroll | `./run.sh enroll <n>` | Sign up n accounts and append every key to data.txt |
| bg | `./run.sh bg` | Same as run but in background with log file |
| bgprobe | `./run.sh bgprobe` | Probe in background |
| screen | `./run.sh screen` | Same as run in a detached screen session |
| probe | `./run.sh probe` | Check the site, the solver, and every proxy |
| selftest | `./run.sh selftest` | Offline checks, no network |
| stop | `./run.sh stop` | Stop background or screen session |
| build | `./run.sh build` | Build only |

---

## Download Prebuilt Binary

Prebuilt binaries are automatically compiled on every push to main via GitHub release across 3 platforms.

Download the latest binaries from the release page:
https://github.com/Yuurisan-N1/Token-Harbor/releases/tag/v1.0.0

Open the latest successful run and scroll to the Artifacts section.

| Artifact | Platform |
|---|---|
| `tokenharbor-bot-linux-x86_64` | Linux x86_64 |
| `tokenharbor-bot-windows-x86_64` | Windows x86_64 |
| `tokenharbor-bot-macos-aarch64` | macOS Apple Silicon |

**Linux / macOS after downloading:**

```bash
chmod +x tokenharbor-bot-linux-x86_64
./tokenharbor-bot-linux-x86_64
```

**Windows:**

```
.\tokenharbor-bot-windows-x86_64.exe
```

Place the binary in the same folder as `data.txt`, `proxy.txt`, `config.json`, and `solver/solver.yaml` before running.

---

## Features

### Auto Signup via Disposable Mailbox
When no key exists in `data.txt` or enroll mode is used, the bot opens a disposable mailbox automatically through one of four providers tried in order: mail.tm, mail.io, mail.lol, and Guerrilla Mail. It generates a random email address with the `th-` prefix, submits the signup form with a fresh password and the account timezone, and waits for the verification email.

### Turnstile Solver
Every signup request includes a Cloudflare Turnstile token solved automatically through the endpoint in `solver/solver.yaml`. The solver handshake is verified on startup and in probe mode.

### Email Verification
The bot polls the inbox after signup and follows the verification link when the email arrives. If the first polling window times out, the bot retries before giving up.

### Free Model Unlock
After verification, the bot enables the free model tier on the account so the API key can access models without credit.

### API Key Creation
A named API key is created under the account and appended to `data.txt`. The key is logged in masked form. The account email, password, and key are saved to `session.json` for reference.

### Chat and Usage Report
On every run cycle the bot reads the model list, picks a free model, sends one short chat to keep the account active, and then reads the usage headers from the server. The used percentage and the reset countdown are logged per account.

### Enroll Mode
Running `enroll <n>` signs up exactly n accounts one by one, writing each key to `data.txt` as it completes. Each account uses its own proxy slot from `proxy.txt`.

### Probe Mode
Probe checks the signup page, the model list endpoint, the solver handshake, and the exit country of every proxy in `proxy.txt` without signing up or modifying any files.

### Proxy Support
Proxies are loaded from `proxy.txt` and assigned to accounts by position. Running without proxies is fully supported.

### Auto Countdown
After each cycle completes the bot shows a live `HH:MM:SS` countdown until the next cycle starts.

---

## File Structure

```text
Token-Harbor/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 3-platform matrix build
├── assets/
│   └── icon.ico                         # Windows binary icon
├── solver/
│   └── solver.yaml                      # Turnstile solver endpoint URL
├── src/
│   ├── cli/
│   │   ├── dispatch.rs                  # CLI mode selector and usage text
│   │   ├── enroll.rs                    # Enroll mode: batch account signup
│   │   ├── mod.rs
│   │   ├── probe.rs                     # Probe mode: site, solver, and proxy check
│   │   └── selftest.rs                  # Offline self-test checks
│   ├── job/
│   │   ├── loop.rs                      # Cycle runner for run and enroll modes
│   │   ├── mod.rs
│   │   └── onboard.rs                   # Signup flow, API key creation, chat and usage report
│   ├── net/
│   │   ├── client.rs                    # HTTP client and cookie handling
│   │   ├── egress.rs                    # Proxy parser and egress country check
│   │   ├── gate.rs                      # Turnstile solver WebSocket gate
│   │   └── mod.rs
│   ├── step/
│   │   ├── apikey.rs                    # API key creation request
│   │   ├── freemode.rs                  # Free model tier enable request
│   │   ├── mailbox.rs                   # Disposable mailbox: mail.tm, mail.io, mail.lol, Guerrilla
│   │   ├── mod.rs
│   │   ├── signup.rs                    # Account registration form and identity generation
│   │   └── verify.rs                    # Email verification link follow and confirm
│   ├── support/
│   │   ├── banner.rs                    # ASCII banner and terminal title
│   │   ├── cfg/
│   │   │   ├── loader.rs                # Config, key list, and proxy loader
│   │   │   ├── mod.rs
│   │   │   └── schema.rs                # Config schema and sleep/timezone settings
│   │   ├── consts/
│   │   │   ├── app.rs                   # File name constants and account generation params
│   │   │   ├── mod.rs
│   │   │   └── net.rs                   # Base URL, solver path, and mail service URLs
│   │   ├── error.rs                     # Error types and result alias
│   │   ├── log/
│   │   │   ├── format.rs                # Colored log helpers and countdown
│   │   │   ├── init.rs                  # Signal handler setup
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── wire/
│   │   ├── endpoint.rs                  # API endpoint path constants
│   │   ├── form.rs                      # JSON request body builders
│   │   ├── mod.rs
│   │   └── shape.rs                     # JSON response parsers
│   └── main.rs                          # Entry point, Ctrl+C handler, root dir resolver
├── .gitignore
├── build.rs                             # Build script: embeds icon into Windows binary
├── Cargo.lock
├── Cargo.toml                           # Project manifest and dependencies
├── config.json                          # Sleep duration and optional timezone setting
├── data.txt                             # API keys, one per line (written by signup and enroll)
├── HOWTO.txt                            # Quick start guide
├── LICENSE                              # GNU AGPL v3.0
├── Makefile                             # make targets: build, release, run, start, clean, size
├── proxy.txt                            # Proxy list (optional)
├── README.md                            # This file
├── run.sh                               # Run helper: run, enroll, bg, bgprobe, screen, probe, selftest, stop, build
└── session.json                         # Account email, password, and key cache (auto-generated)
```

---

## Continuous Integration

The GitHub Actions workflow runs on pushes to `main` or `master` and on manual dispatch. It builds the release binary on all three platforms and uploads the artifacts.

| Platform | Artifact |
|---|---|
| Linux | `tokenharbor-bot-linux-x86_64` |
| Windows | `tokenharbor-bot-windows-x86_64` |
| macOS | `tokenharbor-bot-macos-aarch64` |

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>