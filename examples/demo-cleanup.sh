#!/bin/bash
# Cleanup script for SpaceComms demo

echo "Cleaning up SpaceComms demo processes..."

# Kill any running spacecomms processes
pkill -f "spacecomms start" 2>/dev/null || true

# Remove log files
rm -f /tmp/node-a.log /tmp/node-b.log

echo "Done."
