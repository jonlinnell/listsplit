# listsplit

A simple tool for making long lists of items less horrifying to look at.

## Options

`-i`, `--indent-depth <INDENT_DEPTH>`
The number spaces to insert before each line [default: 4]

`-c`, `--max-elements-per-line <MAX_ELEMENTS_PER_LINE>`
The number of elements to display per line [default: 5]

`-h`, `--help`
Print help

`-V`, `--version`
Print version

## Example

```sh
echo '1,2,3,4,5,6,7,8,9,10,11,12,13,14' | listsplit -c 4 -i 2

# =>
#   1,2,3,4,
#   5,6,7,8,
#   9,10,11,12,
#   13,14
```
