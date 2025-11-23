# metadata-explorer

# Building

```Shell
cargo build --release

#after tool in target directory
```

# Usage

```Shell
mde --help #for show file types help
mde <file_type_flag> --help #for show help per file type

#Examples

#reading test.png
mde --png --read --filename test.png

#inserting tag hABr in offset 0 with data "Hello Habr"
mde --png --write --filename test.png --offset 0 --chunk_type hABr --data 72,101,108,108,111,32,72,97,98,114 
```
