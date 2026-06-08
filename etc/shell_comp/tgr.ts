const completion: Fig.Spec = {
  name: "tgr",
  description: "Manage `.org` files' tags from the CLI",
  subcommands: [
    {
      name: ["count", "c"],
      description: "Print the number of tags that match <pattern>",
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
          name: ["-i", "--include"],
          description: "Override config by including files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "include",
            isOptional: true,
          },
        },
        {
          name: ["-e", "--exclude"],
          description: "Override config by excluding files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "exclude",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: {
        name: "pattern",
        isOptional: true,
      },
    },
    {
      name: ["locate", "l", "loc"],
      description: "Locate the files that contain a tag matching <PATTERN>",
      options: [
        {
          name: ["-i", "--include"],
          description: "Override config by including files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "include",
            isOptional: true,
          },
        },
        {
          name: ["-e", "--exclude"],
          description: "Override config by excluding files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "exclude",
            isOptional: true,
          },
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
      args: {
        name: "pattern",
      },
    },
    {
      name: ["refile", "r", "ref"],
      description: "Refile org trees that have tags that match a pattern",
      options: [
        {
          name: ["-i", "--include"],
          description: "Override config by including files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "include",
            isOptional: true,
          },
        },
        {
          name: ["-e", "--exclude"],
          description: "Override config by excluding files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "exclude",
            isOptional: true,
          },
        },
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
          name: ["-i", "--include"],
          description: "Override config by including files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "include",
            isOptional: true,
          },
        },
        {
          name: ["-e", "--exclude"],
          description: "Override config by excluding files that match <PATTERN>",
          isRepeatable: true,
          args: {
            name: "exclude",
            isOptional: true,
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
          name: ["-P", "--print"],
          description: "Print all the tags into a .txt file",
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
          name: "count",
          description: "Print the number of tags that match <pattern>",
        },
        {
          name: "locate",
          description: "Locate the files that contain a tag matching <PATTERN>",
        },
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
