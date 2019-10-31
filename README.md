## CLI usage
`memoparsa [-o/--output output_file] [input_file]`
or
`cargo run -- [-o/--output output_file] [input_file]`.

Output defines file to be output to, currently defaults to `parsa.txt`. Input file must be defined by user.
After collecting this information, the current iteration just prints it out. Future iteration will take the origin file,
run the parser on it, then create a file at default or set output with the parsed contents.

## Logging
`RUST_LOG=memoparsa_rs=trace`

## Trello
[Trello board for feature tracking](https://trello.com/b/63wxxS5e/memoparsa)

