const completion: Fig.Spec = {
  name: "tgr",
  description: "Manage `.org` files' tags from the CLI",
  subcommands: [
    {
      name: ["refile", "r", "ref"],
      description: "Refile org trees that have tags that match a pattern",
      options: [
        {
          name: ["-n", "--no-pager"],
          description: "Print the contents to stdout instead of pager",
        },
        {
          name: ["-s", "--strict"],
          description: "Match the pattern strictly or loosely",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: [
        {
          name: "pattern",
        },
        {
          name: "output_file",
          isOptional: true,
        },
      ]
    },
    {
      name: ["search", "s"],
      description: "Search tags in Org directory or file",
      options: [
        {
          name: ["-f", "--file"],
          description: "File where to search for tags",
          isRepeatable: true,
          args: {
            name: "file",
            isOptional: true,
            template: "filepaths",
          },
        },
        {
          name: ["-p", "--pager"],
          description: "Force the output to a pager",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: {
        name: "pattern",
      },
    },
    {
      name: ["tags", "t", "tag"],
      description: "Print tags to stdout or to pager",
      options: [
        {
          name: ["-f", "--file"],
          description: "Optional file to search instead of searching in the whole Org directory",
          isRepeatable: true,
          args: {
            name: "file",
            isOptional: true,
            template: "filepaths",
          },
        },
        {
          name: ["-p", "--pager"],
          description: "Force the output to a pager",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "refile",
          description: "Refile org trees that have tags that match a pattern",
        },
        {
          name: "search",
          description: "Search tags in Org directory or file",
        },
        {
          name: "tags",
          description: "Print tags to stdout or to pager",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help (see more with '--help')",
    },
    {
      name: ["-V", "--version"],
      description: "Print version",
    },
  ],
};

export default completion;
