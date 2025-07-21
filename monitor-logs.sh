#!/bin/bash

# Discord TTS Bot Log Monitor
# Real-time log monitoring with color coding and filtering

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOGS_DIR="$SCRIPT_DIR/logs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

print_usage() {
    echo "Discord TTS Bot Log Monitor"
    echo ""
    echo "Usage: $0 [option]"
    echo ""
    echo "Options:"
    echo "  all       - Monitor all logs (default)"
    echo "  bot       - Monitor bot logs only"
    echo "  xtts      - Monitor XTTS logs only"
    echo "  database  - Monitor database logs only"
    echo "  errors    - Monitor all error logs"
    echo "  grep <pattern> - Monitor logs matching pattern"
    echo ""
    echo "Examples:"
    echo "  $0              # Monitor all logs"
    echo "  $0 bot          # Monitor bot logs only"
    echo "  $0 grep ERROR   # Show only lines containing ERROR"
    echo ""
    echo "Press Ctrl+C to stop monitoring"
}

monitor_all() {
    echo -e "${GREEN}Monitoring all logs...${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"
    
    # Use tail with color coding for different log types
    tail -f logs/*.log | while read line; do
        if [[ $line == *"logs/bot.log"* ]]; then
            echo -e "${BLUE}[BOT]${NC} ${line#*]==>}"
        elif [[ $line == *"logs/xtts.log"* ]]; then
            echo -e "${PURPLE}[XTTS]${NC} ${line#*]==>}"
        elif [[ $line == *"logs/database.log"* ]]; then
            echo -e "${CYAN}[DB]${NC} ${line#*]==>}"
        elif [[ $line == *".error.log"* ]]; then
            echo -e "${RED}[ERROR]${NC} ${line#*]==>}"
        else
            echo "$line"
        fi
    done
}

monitor_single() {
    local service=$1
    local log_file="$LOGS_DIR/${service}.log"
    local error_file="$LOGS_DIR/${service}.error.log"
    
    echo -e "${GREEN}Monitoring $service logs...${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"
    
    if [ -f "$log_file" ] || [ -f "$error_file" ]; then
        tail -f "$log_file" "$error_file" 2>/dev/null | while read line; do
            if [[ $line == *".error.log"* ]]; then
                echo -e "${RED}[ERROR]${NC} ${line#*]==>}"
            else
                echo "$line"
            fi
        done
    else
        echo -e "${RED}Log files not found for $service${NC}"
        exit 1
    fi
}

monitor_errors() {
    echo -e "${RED}Monitoring error logs only...${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"
    
    tail -f logs/*.error.log | while read line; do
        echo -e "${RED}[ERROR]${NC} $line"
    done
}

monitor_grep() {
    local pattern=$1
    echo -e "${GREEN}Monitoring logs for pattern: $pattern${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"
    
    tail -f logs/*.log | grep --line-buffered "$pattern" | while read line; do
        echo -e "${YELLOW}[MATCH]${NC} $line"
    done
}

# Clear screen for better visibility
clear

# Main logic
case "$1" in
    all|"")
        monitor_all
        ;;
    bot)
        monitor_single "bot"
        ;;
    xtts)
        monitor_single "xtts"
        ;;
    database)
        monitor_single "database"
        ;;
    errors)
        monitor_errors
        ;;
    grep)
        if [ -z "$2" ]; then
            echo -e "${RED}Error: grep requires a pattern${NC}"
            echo "Example: $0 grep ERROR"
            exit 1
        fi
        monitor_grep "$2"
        ;;
    -h|--help|help)
        print_usage
        ;;
    *)
        echo -e "${RED}Unknown option: $1${NC}"
        print_usage
        exit 1
        ;;
esac