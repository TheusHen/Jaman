# Jaman Installation Script for Windows (PowerShell)
# 
# Usage: irm https://raw.githubusercontent.com/TheusHen/jaman/main/install.ps1 | iex

param(
    [string]$InstallDir = "$env:USERPROFILE\Downloads\jaman",
    [string]$Repo = "TheusHen/jaman"
)

$ErrorActionPreference = "Stop"

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Get-LatestVersion {
    Write-ColorOutput "Fetching latest version..." "Cyan"
    
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $version = $response.tag_name -replace '^v', ''
        Write-ColorOutput "Latest version: v$version" "Green"
        return $version
    }
    catch {
        Write-ColorOutput "Failed to fetch latest version: $_" "Red"
        exit 1
    }
}

function Install-Jaman {
    param([string]$Version)
    
    $assetName = "jaman-windows-x64.exe"
    $downloadUrl = "https://github.com/$Repo/releases/download/v$Version/$assetName.zip"
    $tempFile = "$env:TEMP\jaman.zip"
    
    Write-ColorOutput "`nDownloading from $downloadUrl..." "Cyan"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile -UseBasicParsing
    }
    catch {
        Write-ColorOutput "Download failed: $_" "Red"
        exit 1
    }
    
    Write-ColorOutput "Creating installation directory: $InstallDir" "Cyan"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    
    Write-ColorOutput "Extracting..." "Cyan"
    Expand-Archive -Path $tempFile -DestinationPath $InstallDir -Force
    
    Write-ColorOutput "Cleaning up..." "Cyan"
    Remove-Item $tempFile -Force
}

function Add-ToPath {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($currentPath -notlike "*$InstallDir*") {
        Write-ColorOutput "`nAdding to PATH..." "Cyan"
        
        $newPath = "$currentPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        
        # Update current session
        $env:Path = "$env:Path;$InstallDir"
        
        Write-ColorOutput "Added to PATH" "Green"
    }
    else {
        Write-ColorOutput "`nPATH already configured" "Yellow"
    }
}

function Test-Installation {
    $exePath = Join-Path $InstallDir "jaman.exe"
    
    if (Test-Path $exePath) {
        Write-ColorOutput "`n========================================" "Green"
        Write-ColorOutput "✓ Jaman installed successfully!" "Green"
        Write-ColorOutput "========================================" "Green"
        Write-ColorOutput "`nInstallation location: $exePath" "Cyan"
        Write-ColorOutput "`nTo start using jaman, either:" "White"
        Write-ColorOutput "  1. Restart your terminal, or" "White"
        Write-ColorOutput "  2. Close and reopen PowerShell" "White"
        Write-ColorOutput "`nThen run: jaman --version" "Cyan"
        Write-ColorOutput "`nGet started with: jaman --help" "Cyan"
        Write-ColorOutput ""
        
        return $true
    }
    else {
        Write-ColorOutput "Installation failed - executable not found" "Red"
        return $false
    }
}

# Main installation flow
function Main {
    Write-ColorOutput "==========================================" "Cyan"
    Write-ColorOutput "  Jaman Installer for Windows" "Cyan"
    Write-ColorOutput "==========================================" "Cyan"
    Write-ColorOutput ""
    
    $version = Get-LatestVersion
    Install-Jaman -Version $version
    Add-ToPath
    
    if (Test-Installation) {
        Write-ColorOutput "Installation complete!" "Green"
    }
    else {
        Write-ColorOutput "Installation incomplete. Please try again." "Red"
        exit 1
    }
}

# Run installation
Main
