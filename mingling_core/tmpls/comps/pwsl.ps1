Register-ArgumentCompleter -CommandName <<<bin_name>>> -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $line = $commandAst.ToString()
    $commandName = if ($commandAst.CommandElements.Count -gt 0) {
        $commandAst.CommandElements[0].Value
    } else { "" }

    $words = @()
    $currentIndex = 0
    $parser = [System.Management.Automation.PSParser]
    $tokens = $parser::Tokenize($line, [ref]$null)

    foreach ($token in $tokens) {
        if ($token.Type -in 'CommandArgument', 'CommandParameter') {
            $words += $token.Content
        }
    }

    $args = @(
        "-f", ($line -replace '-', '^')
        "-C", $cursorPosition.ToString()
        "-w", ($wordToComplete -replace '-', '^')
        "-p", (if ($words.Count -gt 1) { $words[-2] } else { "" }) -replace '-', '^'
        "-c", $commandName
        "-i", ($words.Count - 1).ToString()
        "-a", ($words | ForEach-Object { $_ -replace '-', '^' })
        "-F", "powershell"
    )

    $suggestions = <<<bin_name>>> __comp $args 2>$null

    if ($suggestions) {
        $suggestions | ForEach-Object {
            if ($_ -eq "_file_") {
                $completionType = 'ProviderItem'
            } else {
                $completionType = 'ParameterValue'
            }
            [System.Management.Automation.CompletionResult]::new($_, $_, $completionType, $_)
        }
    }
}
