# dispath
Display the content of PATH-like environment variables, printing one entry per line on standard output.

```
Usage: dispath [<vars...>] [-r <regex>] [-u] [-a] [-s <sep>] [--fail-unset]

Display PATH-like environment variables, one entry per line.

Positional Arguments:
  vars              variables to display. default=["PATH"]

Options:
  -r, --regex       filter entries to display with a regex. default=".*"
  -u, --unique      do not print the same entry twice (preserves entry order)
  -a, --all-vars    display all set variables instead of vars
  -s, --sep         entry separator. default=':'
  --fail-unset      fail if vars are unset
  --help            display usage information
```

### Common use cases
##### show `PATH` content  
`dispath`

##### show `LD_LIBRARY_PATH`  
`dispath LD_LIBRARY_PATH`

##### show entries in `LD_LIBRARY_PATH` that starts with `/`  
`dispath LD_LIBRARY_PATH -r '^/'`

##### show entries in `PATH` and `PKG_CONFIG_PATH` that are [Nix](https://nixos.org/) store paths; fail if any is unset  
`dispath PATH PKG_CONFIG_PATH -r '^/nix/store/' --fail-unset `

##### show all entries that look like a [Nix](https://nixos.org/) store path in all environment variables, omitting repeated entries  
`dispath -r '^/nix/store/' --all-vars --unique`
