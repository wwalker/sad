# SAD!

Super Accelerated Diff

## What does it do?

Basically `sad` is a **Batch Search and Replace** tool.

It will show you a really nice preview however.

Unlike `sed`, you can double check before you fat finger your edit.

## How to use sad?

```sh
find "$FIND_ARGS" | sad '<pattern>' '<replacement>' | highlighter-of-your-choice
```

Feed `sad` a list of files from `stdin`, a search pattern (regex by default), a replacement pattern, and you are good to go!

You can use regex capture groups. For example: `sad '"(\d+)"' ''|$1|''` will replace the double quotes around integers with `|`.

If a replacement pattern is omitted, `sad` will assume deletion.

## Requirements

`sad` is designed to work with a diff colorizer. Any would work.

My recommendations are:

[diff-so-fancy](https://github.com/so-fancy/diff-so-fancy) `perl`

[delta](https://github.com/dandavison/delta) `rust`

`sad`'s preview mode `stdout` is not designed to be read by humans.

## Flags

Name                                | Function
----------------------------------- | -------------------------------------------------------------------
`-i file1 file2` `--input files...` | instead of reading from `stdin`, read file names from argument list
`-k --commit`                       | instead of printing out a preview, write edits to files
`-0`                                | use `\0` instead of `\n` when reading from `stdin`
`-e` `--exact`                      | use string literal match instead of regex
`-f isx` `--flags mI`               | flags for the regex engine

## Regex Flags

Name | Function
-----|----------------------------------------------------
`i`  | case insensitive (works for `--exact` mode as well)
`I`  | case sensitive (works for `--exact` mode as well)
`m`  | multiline: `^` `$` match each line
`s`  | allow `.` match `\n`
`x`  | ignore whitespace and allow `#` comments

## GET SAD NOW!

You can download `sad` from the [github release page](https://github.com/ms-jpq/sad/releases).