# Auto-Startup Guide for Discord TTS Bot on macOS

This guide explains how to configure your Discord TTS Bot to start automatically when your Mac restarts.

## 🚀 Quick Setup

```bash
# Install all services for automatic startup
./manage-startup.sh install

# Check status
./manage-startup.sh status
```

## 📋 Prerequisites

1. **Completed Native XTTS Setup** (see NATIVE-XTTS-SETUP.md)
2. **Docker Desktop** installed and configured to start at login
3. **Release binary built** (`cargo build --release`)
4. **Valid config.toml** in project root

## 🛠 Components

The auto-startup system manages three services:

1. **PostgreSQL Database** (Docker container)
2. **Native XTTS Server** (Python/conda)
3. **Discord Bot** (Rust binary)

## 📁 File Structure

```
Discord-TTS-Bot/
├── launchd/
│   ├── com.discord-tts-bot.database.plist
│   ├── com.discord-tts-bot.xtts.plist
│   └── com.discord-tts-bot.bot.plist
├── logs/
│   ├── database.log
│   ├── xtts.log
│   └── bot.log
└── manage-startup.sh
```

## 🔧 Installation

### 1. Build Release Binary
```bash
cargo build --release
```

### 2. Install Services
```bash
./manage-startup.sh install
```

This will:
- Create logs directory
- Copy plist files to `~/Library/LaunchAgents/`
- Load services into launchd
- Services will start automatically on next login

### 3. Start Services Now
```bash
./manage-startup.sh start
```

## 📊 Management Commands

### Check Status
```bash
./manage-startup.sh status
```

Output example:
```
Discord TTS Bot Services Status:
--------------------------------
com.discord-tts-bot.database: Running (PID: 12345)
com.discord-tts-bot.xtts: Running (PID: 12346)
com.discord-tts-bot.bot: Running (PID: 12347)

Additional checks:
PostgreSQL Docker: Running
XTTS Server: Accessible (http://localhost:8000)
```

### View Logs
```bash
./manage-startup.sh logs
```

### Control Individual Services
```bash
# Start/stop/restart specific service
./manage-startup.sh start database
./manage-startup.sh stop xtts
./manage-startup.sh restart bot
```

### Uninstall Auto-Startup
```bash
./manage-startup.sh uninstall
```

## 🔄 Service Dependencies

Services start in order with delays:
1. **Database** starts immediately
2. **XTTS** starts after 30 seconds
3. **Bot** starts after 2 minutes

## 🛡 Service Features

### Automatic Restart
- Services restart automatically if they crash
- KeepAlive ensures continuous operation
- Configurable restart intervals

### Logging
- All output redirected to log files
- Separate error logs for debugging
- Logs stored in `./logs/` directory

### Environment Variables
- PATH configured for all tools
- Conda environment activated for XTTS
- Custom environment variables supported

## 🔍 Troubleshooting

### Service Won't Start
1. Check logs: `./manage-startup.sh logs`
2. Verify prerequisites:
   ```bash
   # Check Docker
   docker ps
   
   # Check conda
   conda info
   
   # Check release binary
   ls -la target/release/tts_bot
   ```

### Permission Issues
```bash
# Fix permissions
chmod +x manage-startup.sh
chmod 644 launchd/*.plist
```

### XTTS Fails to Start
1. Ensure conda is installed: `which conda`
2. Check conda environment: `conda env list | grep xtts`
3. Verify Python path in plist file

### Database Connection Issues
1. Ensure Docker Desktop is running
2. Check Docker is set to start at login
3. Verify database container: `docker ps | grep postgres`

### Manual Service Control
```bash
# Load service manually
launchctl load ~/Library/LaunchAgents/com.discord-tts-bot.bot.plist

# Check if service is loaded
launchctl list | grep discord-tts-bot

# View service details
launchctl print gui/$(id -u)/com.discord-tts-bot.bot
```

## 🔧 Customization

### Modify Startup Order
Edit StartInterval in plist files:
```xml
<key>StartInterval</key>
<integer>60</integer> <!-- Delay in seconds -->
```

### Change Log Locations
Edit StandardOutPath/StandardErrorPath in plist files:
```xml
<key>StandardOutPath</key>
<string>/path/to/custom/log.log</string>
```

### Add Environment Variables
Add to EnvironmentVariables in plist files:
```xml
<key>EnvironmentVariables</key>
<dict>
    <key>CUSTOM_VAR</key>
    <string>value</string>
</dict>
```

## 🚨 Important Notes

1. **Docker Desktop** must be configured to start at login
2. **First startup** after reboot may take 2-3 minutes
3. **Logs** can grow large - consider log rotation
4. **Updates** require rebuilding release binary

## 📝 Maintenance

### Update Bot Code
```bash
# Stop services
./manage-startup.sh stop

# Update and rebuild
git pull
cargo build --release

# Restart services
./manage-startup.sh start
```

### View Service Logs in Real-Time
```bash
# Follow all logs
tail -f logs/*.log

# Follow specific service
tail -f logs/bot.log
```

### Clean Logs
```bash
# Archive old logs
tar -czf logs-backup-$(date +%Y%m%d).tar.gz logs/
rm logs/*.log
```

## 🎯 Verification

After installation and Mac restart:

1. Wait 2-3 minutes for all services to start
2. Run `./manage-startup.sh status`
3. Check Discord bot is online
4. Test TTS functionality

The bot should now start automatically every time your Mac restarts!