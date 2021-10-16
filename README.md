# unembedx

Utility for quick extraction of embedded files from Office Open XML
(OOXML) files.

Specifically, it unzips the document, looks through any [compound
files](https://en.wikipedia.org/wiki/Compound_File_Binary_Format]) in
the embeddings directory and extracts anything interesting into a
series of numbered files. By default it will also attempt to determine
the correct file extensions from the content of each file using
libmagic.

If you're interested in something other than the files mentioned
above, you can try running unzip on the document rather than this
utility.

The current version has only been tested by extracting pdf files from
PowerPoint presentations and might not handle your specific use
case. Bug reports and pull requests are welcome.

## Building

Install rust and run
```
cargo build --release
```

The resulting executable can be found at target/release/unembedx.

By default, compilation requires `file-devel` (fedora), `libmagic-dev`
(debian, ubuntu) or equivalent. This allows unembedx to automatically
append the correct file extension to the extracted files in some
cases.

To build without the file extension logic, you can instead run
```
cargo build --release --no-default-features
```

## Usage

Extract all embedded files into the current directory
```
./unembedx some-presentation.pptx
```

Use `--help` for more information.

## License

MIT
