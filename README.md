# prose_dl -- **See #3**

Helper to download all posts from prose.sh instance

[![dependency status](https://deps.rs/repo/gitea/git.mylloon.fr/Anri/prose_dl/status.svg)](https://deps.rs/repo/gitea/git.mylloon.fr/Anri/prose_dl)

## Installation

Clone the repo and get in

```bash
git clone https://git.mylloon.fr/Anri/prose_dl.git && cd prose_dl
```

Then install it

```bash
cargo install --path .
```

## Usage

Will download all your posts from [`prose.sh`](https://prose.sh/) into a
folder named after your username:

```bash
$ prose_dl <username>
```

Will download the special files too:

```bash
$ prose_dl -s <username>
```

More info with the `--help` option.

## FAQ

> Can this download unlisted posts (thoses with _date metadata_ set in the future)?

- No.

> Can this download specials files?

- Yes, with `-s` option

> Can I use a custom domain?

- Yes, with `--domain` option
