A CLI tool for make blogging faster.

This is a command line tool that wraps the `Hugo new` command.

It performs the following operations,

- checkout the article title name branch
- create `index.md` in `contents/posts/<article-name>/`

**TODO**

- [x] Checkout the article title branch.
- [ ] Open with Vim.
- [ ] Start a local server in the background while editing.
- [ ] Env values support for create new post everywhere
