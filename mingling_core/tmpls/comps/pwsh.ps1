# PowerShell completion script for <<<bin_name>>>
Register-ArgumentCompleter -Native -CommandName '<<<bin_name>>>' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $words = $commandAst.ToString().Split(' ')
    $currentIndex = $words.IndexOf($wordToComplete)
    if ($currentIndex -eq -1) { $currentIndex = $words.Count }

    $buffer = $commandAst.ToString()
    $currentWord = $wordToComplete
    $previousWord = if ($currentIndex -gt 1) { $words[$currentIndex - 2] } else { "" }
    $commandName = if ($words.Count -gt 0) { $words[0] } else { "" }
    $wordIndex = $currentIndex

    $args = @(
        "-f", $buffer.Replace('-', '^')
        "-C", $cursorPosition
        "-w", $currentWord.Replace('-', '^')
        "-p", $previousWord.Replace('-', '^')
        "-c", $commandName
        "-i", $wordIndex
        "-a", ($words | ForEach-Object { $_.Replace('-', '^') }) -join ' '
        "-F", "pwsh"
    )

    $suggestions = & <<<bin_name>>> __comp $args 2>$null

    if ($LASTEXITCODE -eq 0 -and $suggestions) {
        $completions = $suggestions -split "`n"

        if ($completions[0].Trim() -eq "_file_") {
            $completions = if ($completions.Count -gt 1) {
                $completions[1..($completions.Count-1)]
            } else {
                @()
            }

            $completions | ForEach-Object {
                $path = $_.Replace('^', '-')
                $isDirectory = $path.EndsWith([System.IO.Path]::DirectorySeparatorChar) -or $path.EndsWith('/')
                $completionType = if ($isDirectory) { 'ProviderContainer' } else { 'ProviderItem' }
                [System.Management.Automation.CompletionResult]::new($path, $path, $completionType, $path)
            }
        }
        else {
            $parsedCompletions = @()
            foreach ($item in $completions) {
                if ($item -match '^([^$]+)\$\((.+)\)$') {
                    $parsedCompletions += "$($matches[1]):$($matches[2])"
                }
                else {
                    $parsedCompletions += $item
                }
            }

            $simpleCompletions = @()
            foreach ($item in $parsedCompletions) {
                if ($item -match '^([^:]+):(.+)$') {
                    $simpleCompletions += $matches[1]
                }
                else {
                    $simpleCompletions += $item
                }
            }

            return $simpleCompletions | ForEach-Object {
                [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
            }
        }
    }
}
