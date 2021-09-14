#!/bin/sh

curl -sSL https://github.com/Steven-Torres/minigrep/releases/latest/download/minigrep --output /usr/local/bin/minigrep

chmod +x /usr/local/bin/minigrep

echo "mingrep installed to '/usr/local/bin/minigrep'"
echo "Run 'minigrep --help' to get started"