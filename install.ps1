Invoke-WebRequest `
  -URI "https://github.com/Steven-Torres/minigrep/releases/latest/download/minigrep.exe" `
  -OutFile "C:\Windows\System32"

Write-Output `
  "mingrep installed to 'C:\Windows\System32'" `
  "Run 'minigrep --help' to get started"