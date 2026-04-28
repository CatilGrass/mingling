#!/usr/bin/env pwsh

# Save original directory, restore after execution
$_load_original_dir = Get-Location

# Load completion script mling.ps1 from the current directory
$mlingScript = Join-Path -Path (Get-Location) -ChildPath ".comp/mling_comp.ps1"
if (Test-Path $mlingScript) {
    . $mlingScript
}

# Change to script directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
try {
    Set-Location $scriptPath -ErrorAction Stop
} catch {
    Write-Error "load.ps1: failed to cd to script directory"
    return
}

# Add bin directories from all namespaces to PATH
Get-ChildItem -Directory -Path "*/bin/" | ForEach-Object {
    $env:PATH = "$($_.FullName);$env:PATH"
}

# Helper function: execute script with appropriate shell
function _load_script {
    param([string]$script)
    # Only handle .ps1 scripts
    if ($script -like "*.ps1") {
        & $script 2>$null
    }
}

# Iterate over all namespaces
Get-ChildItem -Directory | ForEach-Object {
    $_namespace = $_.Name

    # Skip if UNTRUSTED marker exists
    if (Test-Path "$_namespace\UNTRUSTED") { return }

    $_comp_dir = "$_namespace\comp"
    if (-not (Test-Path $_comp_dir -PathType Container)) { return }

    # Find all loadable scripts under comp
    $_scripts = Get-ChildItem -Path $_comp_dir -File -Include "*.ps1" -ErrorAction SilentlyContinue
    if (-not $_scripts) { return }

    # Count scripts
    $_count = ($_scripts | Measure-Object).Count

    # If TRUSTED marker exists, load directly
    if (Test-Path "$_namespace\TRUSTED") {
        $_scripts | ForEach-Object {
            _load_script $_.FullName
        }
        return
    }

    # No marker, ask user
    $answer = Read-Host "'$_namespace' has $_count completion script(s) to load, do you trust it? [Y/n] "
    if ($answer -eq "" -or $answer -match "^(y|yes)$") {
        # Mark as TRUSTED
        New-Item -ItemType File -Path "$_namespace\TRUSTED" -Force | Out-Null

        # Ask whether to load immediately
        $load_answer = Read-Host "Load it immediately? [Y/n] "
        if ($load_answer -eq "" -or $load_answer -match "^(y|yes)$") {
            $_scripts | ForEach-Object {
                _load_script $_.FullName
            }
        }
    } else {
        New-Item -ItemType File -Path "$_namespace\UNTRUSTED" -Force | Out-Null
    }
}

# Restore original working directory
try {
    Set-Location $_load_original_dir -ErrorAction Stop
} catch {}

# Cleanup
Remove-Variable -Name _load_original_dir -ErrorAction SilentlyContinue
Remove-Item Function:_load_script -ErrorAction SilentlyContinue
