A CLI tool for make blogging faster.

This is a command line tool that wraps the `hugo new` command.

It performs the following operations,

- checkout the article title name branch
- generate `index.md` in `contents/posts/<article-name>/`

## require

`hugo` and `git`.

### usage

To create new post, run

```
rugo <article-name>
```

### support

Only Linux/macOS, doesn't support Windows.

**TODO**

- [x] Checkout the article title branch.
- [ ] Open with Vim.
- [ ] Start a local server in the background while editing.
- [ ] Env values support for create new post everywhere
