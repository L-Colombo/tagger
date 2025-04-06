
use builtin;
use str;

set edit:completion:arg-completer[tgr] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tgr'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tgr'= {
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand refile 'Refile org trees that have tags that match a pattern'
            cand search 'Search tags in Org directory or file'
            cand tags 'Print tags to stdout or to pager'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'tgr;refile'= {
            cand -s 'Match the pattern strictly or loosely'
            cand --strict 'Match the pattern strictly or loosely'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tgr;search'= {
            cand -f 'File where to search for tags'
            cand --file 'File where to search for tags'
            cand -p 'Force the output to a pager'
            cand --pager 'Force the output to a pager'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tgr;tags'= {
            cand -f 'Optional file to search instead of searching in the whole Org directory'
            cand --file 'Optional file to search instead of searching in the whole Org directory'
            cand -p 'Force the output to a pager'
            cand --pager 'Force the output to a pager'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'tgr;help'= {
            cand refile 'Refile org trees that have tags that match a pattern'
            cand search 'Search tags in Org directory or file'
            cand tags 'Print tags to stdout or to pager'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'tgr;help;refile'= {
        }
        &'tgr;help;search'= {
        }
        &'tgr;help;tags'= {
        }
        &'tgr;help;help'= {
        }
    ]
    $completions[$command]
}
