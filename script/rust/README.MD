# Recovery script

script to parse urls from the `The Great Suspender` extracted `session`

## Dependencies

* rustc
* cargo

## Build

```
cargo build --release
```

## Usage

```
./target/release/rust path/to/the/session.txt [path/to/the/output_file.txt | urls.txt] [delimeter | &uri=]
```
