$starting_dir = Get-Location
Set-Location "docs"
python -m http.server 3000
Set-Location $starting_dir
