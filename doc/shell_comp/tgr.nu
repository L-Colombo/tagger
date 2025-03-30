module completions {

  # Manage `.org` files' tags from the CLI
  export extern tgr [
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Refile org trees that have tags that match a pattern
  export extern "tgr refile" [
    pattern: string           # Pattern to find Org trees to refile
    output_file: string       # Output file
    --strict(-s)              # Match the pattern strictly or loosely
    --help(-h)                # Print help
  ]

  # Search tags in Org directory or file
  export extern "tgr search" [
    pattern: string           # Pattern used to search for tags
    --file(-f): path          # File where to search for tags
    --help(-h)                # Print help
  ]

  # Print tags to stdout or to pager
  export extern "tgr tags" [
    --file(-f): path          # Optional file to search instead of searching in the whole Org directory
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "tgr help" [
  ]

  # Refile org trees that have tags that match a pattern
  export extern "tgr help refile" [
  ]

  # Search tags in Org directory or file
  export extern "tgr help search" [
  ]

  # Print tags to stdout or to pager
  export extern "tgr help tags" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "tgr help help" [
  ]

}

export use completions *
