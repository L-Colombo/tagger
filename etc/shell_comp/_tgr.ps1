
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tgr' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tgr'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'tgr' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('refile', 'refile', [CompletionResultType]::ParameterValue, 'Refile org trees that have tags that match a pattern')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search tags in Org directory or file')
            [CompletionResult]::new('tags', 'tags', [CompletionResultType]::ParameterValue, 'Print tags to stdout or to pager')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tgr;refile' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Match the pattern strictly or loosely')
            [CompletionResult]::new('--strict', '--strict', [CompletionResultType]::ParameterName, 'Match the pattern strictly or loosely')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;search' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'File where to search for tags')
            [CompletionResult]::new('--file', '--file', [CompletionResultType]::ParameterName, 'File where to search for tags')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;tags' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Optional file to search instead of searching in the whole Org directory')
            [CompletionResult]::new('--file', '--file', [CompletionResultType]::ParameterName, 'Optional file to search instead of searching in the whole Org directory')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;help' {
            [CompletionResult]::new('refile', 'refile', [CompletionResultType]::ParameterValue, 'Refile org trees that have tags that match a pattern')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search tags in Org directory or file')
            [CompletionResult]::new('tags', 'tags', [CompletionResultType]::ParameterValue, 'Print tags to stdout or to pager')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tgr;help;refile' {
            break
        }
        'tgr;help;search' {
            break
        }
        'tgr;help;tags' {
            break
        }
        'tgr;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
