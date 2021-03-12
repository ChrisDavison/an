# an - Analyse Notes

[![Rust](https://github.com/ChrisDavison/an/actions/workflows/rust.yml/badge.svg)](https://github.com/ChrisDavison/an/actions/workflows/rust.yml)

A bunch of features for analysing markdown notes that I find myself going back
to over and over again, wrapped up in a single tool.

## Why?

`complexity`, `size`, and `headercount` let me evaluate which notes may contain
too many ideas, and therefore are perhaps suitable for revision for splitting
into a more *zettelkasten* style.

`structure` is useful for being able to quickly get an idea of a note has the
kind of content I'm searching for.

`links` is useful for ensuring that my notes don't degrade over time (either
through me renaming files and thus breaking links, or websites going offline or
redirecting).

`tags` utilises my `tagsearch` library to identify keywords of the pattern
`@[a-zA-Z1-9]+` in the text. I can then loosely group files matching a given set
of tags, e.g. `an tags -t work project -n thesis` to show *work projects* but
not my *thesis*.

`untagged` utilises my `tagsearch` library to identify files which don't contain
any tags.

## Subcommands

### `complexity`

`an complexity <files>...`

A heuristic on the number of headers and lines of content within each header.
Roughly, `sum_of_all_header_levels / num_headers` (e.g. if you have 1 H1, 2 H2,
2 H3, then the sum `1 + 4 (2x2) + 6 (2x3)`, divided by `5 (1+2+2)`. Sorted by
complexity.

### `headercount`

`an headercount <files>...`

How many headers are in the file. Sorted output.

### `size`

`an size <files>...`

Filesize in bytes. Sorted output.

### `structure`

`an structure <files>...`

Table of Contents of each file (list of headers).

### `links`

`an links <files>... [-l|--local]`

Check all links within the file, including check for existance of local links.
`-l` argument to only check local links.

### `tags`

`an tags <files>... [-t <tags-to-match> -n <tags-to-not-match>]`

Show tags for all files. With `-t`, filter to only files including *all* of the
tags to match. With `-n`, filter to files that don't contain *any* of the tags
to not match. `-t` and `-n` can be both used, and are applied sequentially (i.e.
filter all that match, then remove those that we don't want to match).

### `untagged`

`an untagged <files>...`

Show all files that don't have a tag.
