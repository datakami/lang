# Lang - A Language Detection Tool

Lang is a command-line tool written in Rust that detects the language of given text. It reads data from standard input (stdin) and writes results to standard output (stdout).
Dependencies

Lang uses several dependencies to provide its functionality:

    csv: for reading and writing CSV data.
    serde and serde_derive: for data serialization and deserialization.
    whichlang: for detecting languages

# Setup

Firstly, clone the repository:

```bash

git clone https://github.com/yourusername/lang.git

```

Move into the project directory:

```bash

cd lang

```

Ensure you have Rust installed, and then build the project:

```bash

cargo build --release

```

# Usage

The Lang tool expects input in CSV format with two columns: index and text.

You can use it as follows:

```bash

echo "index,text
1,Hello World" | target/release/lang
```

The output will be in CSV format with two columns: index and lang. lang represents the detected language.

