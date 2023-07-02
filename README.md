## Jext - JSON Extractor

Jext is a command-line tool that allows you to extract values from JSON files based on a given key, including support for nested keys.

### Installation

#### Install the binary:
- Download the latest [release]() for your machine
- Unzip the archive and move the binary file `jext` to the directory where you store binary files.
> The directory where binary files are stored depend on the plattform you are running, some examples are `/.bin` or `~/.local/bin`

####  Compile using cargo:
1. Ensure that you have Rust and Cargo installed. If not, you can install them by following the instructions at [rustup.rs](https://rustup.rs/).

2. Clone the repository or download the source code:

   ```shell
   git clone https://github.com/pingu1337/jext.git
   ```

3. Navigate to the project directory:
    ```shell
    cd jext
    ```
4. Build the program using Cargo:
    ```shell
    cargo build --release
    ```
5. The compiled binary will be located at target/release/jext.

6. Move the binary to your bin folder:
    ```shell
    cp target/release/jext <your_bin_directory>/jext
    ```


### Usage
To extract a value from a JSON file, use the following command:
```shell
jext <json_file> <key>
```

- <json_file>: Path to the JSON file from which you want to extract a value.
- \<key>: Key to specify the value you want to extract. For nested keys, separate each level with a dot (.). For example, "address.street".

The program will read the JSON file, extract the value associated with the provided key, and display the value with syntax highlighting.

### Examples
Suppose you have a JSON file named data.json with the following contents:
```json
{
  "name": "John Doe",
  "age": 30,
  "address": {
    "street": "123 Main St",
    "city": "Exampleville"
  }
}
```

To extract the value of the "name" key:
```shell
jext data.json name
```

output:
```json
"John Doe"
```

To extract the value of the nested key "address.street":
```shell
jext data.json address.street
```

Output:
```json
"123 Main St"
```

To extract the address object:
```shell
jext data.json address
```

Output:
```json
{
  "city": "New York",
  "country": "USA",
  "street": "123 Main St"
}
```

### License
This project is licensed under the [MIT License](LICENSE).