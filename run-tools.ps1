Set-Location -Path (Split-Path -Parent $MyInvocation.MyCommand.Path) -ErrorAction Stop

if ($args.Count -eq 0) {
    Write-Host "Available:"
    if (Test-Path "dev_tools/src/bin") {
        $files = Get-ChildItem -Path "dev_tools/src/bin/*.rs"
        foreach ($file in $files) {
            if ($file -is [System.IO.FileInfo]) {
                Write-Host $file.BaseName
            }
        }
    } else {
        Write-Host "Warning: dev_tools/src/bin directory does not exist"
    }
    exit 1
}

$target_bin = $args[0]
$target_file = "dev_tools/src/bin/${target_bin}.rs"

if (-not (Test-Path $target_file)) {
    Write-Host "Error: target file '$target_file' does not exist"
    exit 1
}

cargo run --manifest-path dev_tools/Cargo.toml --bin $args[0]
