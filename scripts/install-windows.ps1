# QX Quick Transform - Windows Installation Script
# LAZYFROG-kindware.dev | MIT License

Write-Host "╔═══════════════════════════════════════════╗"
Write-Host "║  QX Quick Transform - Installation        ║"
Write-Host "║  LAZYFROG-kindware.dev                    ║"
Write-Host "╚═══════════════════════════════════════════╝"
Write-Host ""

$InstallDir = "$env:LOCALAPPDATA\QX"
$StartMenuDir = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs"

# Create install directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# Copy binaries
if (Test-Path "target\release\qx.exe") {
    Copy-Item "target\release\qx.exe" "$InstallDir\qx.exe"
    Write-Host "✓ Installed qx.exe to $InstallDir"
}

if (Test-Path "target\release\qx-gui.exe") {
    Copy-Item "target\release\qx-gui.exe" "$InstallDir\qx-gui.exe"
    Write-Host "✓ Installed qx-gui.exe to $InstallDir"
}

# Copy icon
if (Test-Path "assets\lazyfrog-kindware.ico") {
    Copy-Item "assets\lazyfrog-kindware.ico" "$InstallDir\qx.ico"
    Write-Host "✓ Installed icon"
}

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    Write-Host "✓ Added to PATH"
}

# Create Start Menu shortcut
$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$StartMenuDir\QX Quick Transform.lnk")
$Shortcut.TargetPath = "$InstallDir\qx-gui.exe"
$Shortcut.IconLocation = "$InstallDir\qx.ico"
$Shortcut.Description = "Lightning-fast encoder/decoder/hasher"
$Shortcut.Save()
Write-Host "✓ Created Start Menu shortcut"

Write-Host ""
Write-Host "Installation complete!"
Write-Host "Run 'qx --help' or search for 'QX Quick Transform' in Start Menu."
Write-Host ""
Write-Host "Note: Restart your terminal to use 'qx' command."
