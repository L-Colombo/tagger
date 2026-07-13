
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
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'Print the number of tags that matching a pattern')
            [CompletionResult]::new('locate', 'locate', [CompletionResultType]::ParameterValue, 'Locate the files that contain a tag matching a pattern')
            [CompletionResult]::new('refile', 'refile', [CompletionResultType]::ParameterValue, 'Refile org trees that have tags that match a pattern')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search tags in Org directory or file')
            [CompletionResult]::new('sed', 'sed', [CompletionResultType]::ParameterValue, 'A wrapper around the `sed` cli utility to safely manipulate tags')
            [CompletionResult]::new('tags', 'tags', [CompletionResultType]::ParameterValue, 'Print tags to stdout or to pager')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tgr;count' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'File where to search for tags')
            [CompletionResult]::new('--file', '--file', [CompletionResultType]::ParameterName, 'File where to search for tags')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('--include', '--include', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('--exclude', '--exclude', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;locate' {
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('--include', '--include', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('--exclude', '--exclude', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Match the pattern strictly or loosely')
            [CompletionResult]::new('--strict', '--strict', [CompletionResultType]::ParameterName, 'Match the pattern strictly or loosely')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;refile' {
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('--include', '--include', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('--exclude', '--exclude', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Print the contents to stdout instead of pager')
            [CompletionResult]::new('--no-pager', '--no-pager', [CompletionResultType]::ParameterName, 'Print the contents to stdout instead of pager')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Match the pattern strictly or loosely')
            [CompletionResult]::new('--strict', '--strict', [CompletionResultType]::ParameterName, 'Match the pattern strictly or loosely')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;search' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'File where to search for tags')
            [CompletionResult]::new('--file', '--file', [CompletionResultType]::ParameterName, 'File where to search for tags')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('--include', '--include', [CompletionResultType]::ParameterName, 'Override config by including files that match <PATTERN>')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('--exclude', '--exclude', [CompletionResultType]::ParameterName, 'Override config by excluding files that match <PATTERN>')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Force the output to a pager')
            [CompletionResult]::new('--pager', '--pager', [CompletionResultType]::ParameterName, 'Force the output to a pager')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;sed' {
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print additional information about substitutions')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print additional information about substitutions')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;tags' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Optional file to search instead of searching in the whole Org directory')
            [CompletionResult]::new('--file', '--file', [CompletionResultType]::ParameterName, 'Optional file to search instead of searching in the whole Org directory')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Force the output to a pager')
            [CompletionResult]::new('--pager', '--pager', [CompletionResultType]::ParameterName, 'Force the output to a pager')
            [CompletionResult]::new('-P', '-P ', [CompletionResultType]::ParameterName, 'Print all the tags into a .txt file')
            [CompletionResult]::new('--print', '--print', [CompletionResultType]::ParameterName, 'Print all the tags into a .txt file')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'tgr;help' {
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'Print the number of tags that matching a pattern')
            [CompletionResult]::new('locate', 'locate', [CompletionResultType]::ParameterValue, 'Locate the files that contain a tag matching a pattern')
            [CompletionResult]::new('refile', 'refile', [CompletionResultType]::ParameterValue, 'Refile org trees that have tags that match a pattern')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search tags in Org directory or file')
            [CompletionResult]::new('sed', 'sed', [CompletionResultType]::ParameterValue, 'A wrapper around the `sed` cli utility to safely manipulate tags')
            [CompletionResult]::new('tags', 'tags', [CompletionResultType]::ParameterValue, 'Print tags to stdout or to pager')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tgr;help;count' {
            break
        }
        'tgr;help;locate' {
            break
        }
        'tgr;help;refile' {
            break
        }
        'tgr;help;search' {
            break
        }
        'tgr;help;sed' {
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
