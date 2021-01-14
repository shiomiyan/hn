A CLI tool for make blogging faster.

This is a command line tool that wraps the `hugo new` command.

It performs the following operations,

- checkout the article title name branch
- generate `index.md` in `contents/posts/<article-name>/`

## require

`hugo` and `git`.

### usage

```
USAGE:
    rugo [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    new     create new post with git branching
```

**TODO**

- [x] Checkout the article title branch.
- [ ] Open with Vim.
- [ ] Start a local server in the background while editing.
- [ ] Env values support for create new post everywhere
