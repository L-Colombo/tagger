# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_tgr_global_optspecs
	string join \n h/help V/version
end

function __fish_tgr_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_tgr_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_tgr_using_subcommand
	set -l cmd (__fish_tgr_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c tgr -n "__fish_tgr_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c tgr -n "__fish_tgr_needs_command" -s V -l version -d 'Print version'
complete -c tgr -n "__fish_tgr_needs_command" -f -a "refile" -d 'Refile org trees that have tags that match a pattern'
complete -c tgr -n "__fish_tgr_needs_command" -f -a "search" -d 'Search tags in Org directory or file'
complete -c tgr -n "__fish_tgr_needs_command" -f -a "tags" -d 'Print tags to stdout or to pager'
complete -c tgr -n "__fish_tgr_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c tgr -n "__fish_tgr_using_subcommand refile" -s n -l no-pager -d 'Print the contents to stdout instead of pager'
complete -c tgr -n "__fish_tgr_using_subcommand refile" -s s -l strict -d 'Match the pattern strictly or loosely'
complete -c tgr -n "__fish_tgr_using_subcommand refile" -s h -l help -d 'Print help'
complete -c tgr -n "__fish_tgr_using_subcommand search" -s f -l file -d 'File where to search for tags' -r -F
complete -c tgr -n "__fish_tgr_using_subcommand search" -s p -l pager -d 'Force the output to a pager'
complete -c tgr -n "__fish_tgr_using_subcommand search" -s h -l help -d 'Print help'
complete -c tgr -n "__fish_tgr_using_subcommand tags" -s f -l file -d 'Optional file to search instead of searching in the whole Org directory' -r -F
complete -c tgr -n "__fish_tgr_using_subcommand tags" -s p -l pager -d 'Force the output to a pager'
complete -c tgr -n "__fish_tgr_using_subcommand tags" -s h -l help -d 'Print help'
complete -c tgr -n "__fish_tgr_using_subcommand help; and not __fish_seen_subcommand_from refile search tags help" -f -a "refile" -d 'Refile org trees that have tags that match a pattern'
complete -c tgr -n "__fish_tgr_using_subcommand help; and not __fish_seen_subcommand_from refile search tags help" -f -a "search" -d 'Search tags in Org directory or file'
complete -c tgr -n "__fish_tgr_using_subcommand help; and not __fish_seen_subcommand_from refile search tags help" -f -a "tags" -d 'Print tags to stdout or to pager'
complete -c tgr -n "__fish_tgr_using_subcommand help; and not __fish_seen_subcommand_from refile search tags help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
