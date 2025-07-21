#!/bin/bash

# Discord TTS Bot Startup Management Script
# This script manages launchd services for automatic startup on macOS

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LAUNCHD_DIR="$SCRIPT_DIR/launchd"
LOGS_DIR="$SCRIPT_DIR/logs"
LAUNCH_AGENTS_DIR="$HOME/Library/LaunchAgents"

# Service definitions
SERVICES=(
    "com.discord-tts-bot.database"
    "com.discord-tts-bot.xtts"
    "com.discord-tts-bot.bot"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_usage() {
    echo "Usage: $0 {install|uninstall|start|stop|restart|status|logs}"
    echo ""
    echo "Commands:"
    echo "  install    - Install all services for automatic startup"
    echo "  uninstall  - Remove all services from automatic startup"
    echo "  start      - Start all services now"
    echo "  stop       - Stop all services"
    echo "  restart    - Restart all services"
    echo "  status     - Show status of all services"
    echo "  logs       - Show recent logs from all services"
    echo ""
    echo "Individual service control:"
    echo "  $0 {start|stop|restart|status} {database|xtts|bot}"
}

create_logs_dir() {
    if [ ! -d "$LOGS_DIR" ]; then
        echo "Creating logs directory..."
        mkdir -p "$LOGS_DIR"
    fi
}

build_release_binary() {
    echo -e "${YELLOW}Building release binary...${NC}"
    cd "$SCRIPT_DIR"
    cargo build --release
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Release binary built successfully${NC}"
    else
        echo -e "${RED}✗ Failed to build release binary${NC}"
        exit 1
    fi
}

install_services() {
    create_logs_dir
    
    # Create LaunchAgents directory if it doesn't exist
    if [ ! -d "$LAUNCH_AGENTS_DIR" ]; then
        echo "Creating LaunchAgents directory..."
        mkdir -p "$LAUNCH_AGENTS_DIR"
    fi
    
    # Build release binary first
    if [ ! -f "$SCRIPT_DIR/target/release/tts_bot" ]; then
        build_release_binary
    fi
    
    echo -e "${GREEN}Installing Discord TTS Bot services...${NC}"
    
    for service in "${SERVICES[@]}"; do
        plist_file="$LAUNCHD_DIR/$service.plist"
        if [ -f "$plist_file" ]; then
            echo "Installing $service..."
            cp "$plist_file" "$LAUNCH_AGENTS_DIR/"
            launchctl load -w "$LAUNCH_AGENTS_DIR/$service.plist" 2>/dev/null || true
            echo -e "${GREEN}✓ $service installed${NC}"
        else
            echo -e "${RED}✗ $service.plist not found${NC}"
        fi
    done
    
    echo ""
    echo -e "${GREEN}Installation complete!${NC}"
    echo "Services will start automatically on next login."
    echo "To start services now, run: $0 start"
}

uninstall_services() {
    echo -e "${YELLOW}Uninstalling Discord TTS Bot services...${NC}"
    
    for service in "${SERVICES[@]}"; do
        if launchctl list | grep -q "$service"; then
            echo "Stopping and unloading $service..."
            launchctl unload "$LAUNCH_AGENTS_DIR/$service.plist" 2>/dev/null || true
        fi
        
        if [ -f "$LAUNCH_AGENTS_DIR/$service.plist" ]; then
            rm "$LAUNCH_AGENTS_DIR/$service.plist"
            echo -e "${GREEN}✓ $service removed${NC}"
        fi
    done
    
    echo -e "${GREEN}Uninstallation complete!${NC}"
}

start_service() {
    local service=$1
    if [ -z "$service" ]; then
        # Start all services
        for svc in "${SERVICES[@]}"; do
            echo "Starting $svc..."
            launchctl start "$svc" 2>/dev/null || echo -e "${YELLOW}Warning: $svc may already be running${NC}"
        done
    else
        # Start specific service
        service_name="com.discord-tts-bot.$service"
        echo "Starting $service_name..."
        launchctl start "$service_name"
    fi
}

stop_service() {
    local service=$1
    if [ -z "$service" ]; then
        # Stop all services in reverse order
        for (( i=${#SERVICES[@]}-1; i>=0; i-- )); do
            svc="${SERVICES[$i]}"
            echo "Stopping $svc..."
            launchctl stop "$svc" 2>/dev/null || true
        done
    else
        # Stop specific service
        service_name="com.discord-tts-bot.$service"
        echo "Stopping $service_name..."
        launchctl stop "$service_name"
    fi
}

restart_service() {
    stop_service "$1"
    sleep 2
    start_service "$1"
}

show_status() {
    echo -e "${GREEN}Discord TTS Bot Services Status:${NC}"
    echo "--------------------------------"
    
    for service in "${SERVICES[@]}"; do
        if launchctl list | grep -q "$service"; then
            pid=$(launchctl list | grep "$service" | awk '{print $1}')
            if [ "$pid" != "-" ]; then
                echo -e "$service: ${GREEN}Running${NC} (PID: $pid)"
            else
                echo -e "$service: ${RED}Stopped${NC}"
            fi
        else
            echo -e "$service: ${RED}Not installed${NC}"
        fi
    done
    
    echo ""
    echo "Additional checks:"
    # Check if PostgreSQL is accessible
    if docker ps | grep -q "postgres"; then
        echo -e "PostgreSQL Docker: ${GREEN}Running${NC}"
    else
        echo -e "PostgreSQL Docker: ${RED}Not running${NC}"
    fi
    
    # Check if XTTS is accessible
    if curl -s http://localhost:8000/docs > /dev/null 2>&1; then
        echo -e "XTTS Server: ${GREEN}Accessible${NC} (http://localhost:8000)"
    else
        echo -e "XTTS Server: ${RED}Not accessible${NC}"
    fi
}

show_logs() {
    echo -e "${GREEN}Recent logs from all services:${NC}"
    echo "=============================="
    
    if [ -d "$LOGS_DIR" ]; then
        for log_file in "$LOGS_DIR"/*.log; do
            if [ -f "$log_file" ]; then
                echo ""
                echo -e "${YELLOW}--- $(basename "$log_file") ---${NC}"
                tail -n 20 "$log_file" 2>/dev/null || echo "No recent logs"
            fi
        done
    else
        echo "No logs directory found. Services may not have been started yet."
    fi
}

# Main script logic
case "$1" in
    install)
        install_services
        ;;
    uninstall)
        uninstall_services
        ;;
    start)
        start_service "$2"
        ;;
    stop)
        stop_service "$2"
        ;;
    restart)
        restart_service "$2"
        ;;
    status)
        show_status
        ;;
    logs)
        show_logs
        ;;
    *)
        print_usage
        exit 1
        ;;
esac