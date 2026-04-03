<p align="center">
<img src="https://img.icons8.com/ios-filled/500/000000/hammer.png" width="120" alt="NightForge Hammer"/>
</p>

<p align="center">
  <strong style="font-size:24px; color:#ff6b6b;">NightForge</strong>
</p>
<p align="center">
  <em style="color:#cfcfcf;">SSH & FTP Honeypot – Security Research & Threat Intelligence</em>
</p>

# NightForge

NightForge is an advanced SSH/FTP honeypot written in Rust. It is designed for security research, threat intelligence, and training purposes. NightForge simulates interactive sessions, captures authentication attempts, logs commands, and provides a pseudo-filesystem environment for realistic attacker interaction.

---

## Overview

NightForge operates by accepting SSH/FTP-like connections on a specified port. Each session is handled independently and supports multiple concurrent connections.

The honeypot captures:

* Username and password attempts
* Commands executed by the client
* Pseudo-filesystem interactions (`LIST`, `GET`, `PUT`)
* Timestamps and session metadata

All interactions are logged using a thread-safe logging system and can be extended to persistent storage or external monitoring pipelines.

---

## Architecture

```text id="nf-arch01"
NightForge/
├── src/
│   ├── main.rs       # Entry point
│   ├── logger.rs     # Thread-safe session logger
│   ├── parser.rs     # Command parsing engine
│   ├── session.rs    # Session state management
│   ├── fs.rs         # Pseudo-filesystem simulation
```

---

## Module Breakdown

* **logger**
  Handles all session and command logging.

  * Thread-safe event recording
  * Structured log output for analysis

* **parser**
  Interprets incoming client commands.

  * Command classification (`USER`, `PASS`, `LIST`, `GET`, `PUT`)
  * Input normalization and validation

* **session**
  Maintains connection lifecycle and state.

  * Authentication tracking
  * Command history per session
  * Isolation between concurrent sessions

* **fs**
  Simulates a virtual filesystem.

  * Dummy file structures
  * Controlled read/write behavior
  * Realistic attacker interaction responses

---

## Features

* Multi-threaded session handling
* Credential capture (for research and training)
* Command parsing and structured logging
* Pseudo-filesystem emulation
* Real-time console monitoring
* Extensible architecture for additional protocols and behaviors

---

## Execution Flow

```text id="nf-flow01"
Client (Attacker)
        │
TCP Listener (0.0.0.0:2121)
        │
New Thread → Session Created
        │
Command Loop:
    ├─ Parse command
    ├─ Handle authentication
    └─ Handle filesystem interaction
        │
Logger → Store events
        │
Client receives response
```

---

## Installation and Execution

1. Install Rust (version 1.72 or higher)
2. Clone the repository:

   ```bash
   git clone https://github.com/yonathanpy/NightForge.git
   cd NightForge
   ```
3. Build and run:

   ```bash
   cargo run
   ```

---

## Default Configuration

NightForge listens on:

```text id="nf-port"
0.0.0.0:2121
```

---

## Example Interaction

```text id="nf-example"
$ ftp 127.0.0.1 2121
Connected
USER guest
331 Username OK
PASS password
230 Login successful
LIST
file1.txt
file2.log
GET file1.txt
Top Secret Data
PUT newfile.txt HelloWorld
250 File stored
```

All commands and session activity are logged by the logger module.

---

## Extensibility

NightForge can be extended with:

* SIEM or alerting system integration
* Persistent storage (SQLite, PostgreSQL, etc.)
* Advanced filesystem simulation
* Additional command support
* Multi-protocol expansion (SSH, FTP, SFTP, Telnet)

---

## Deployment Considerations

* Run in sandboxed or isolated environments
* Monitor memory and thread usage under load
* Expose only intentionally simulated services
* Use strictly for research, training, or controlled testing

---

## Disclaimer

NightForge is strictly intended for controlled and authorized use. Deploying it outside authorized environments may violate local laws or regulations.
