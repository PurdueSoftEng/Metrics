# ECE461_Team19_CLI

User should run `./run install`, then either `./run build` -> `./run file_name` to run the program or `./run tests` to run tests.

### `./run install`

Installs rustup if not found. Then, installs llvm tools (unless on eceprog).

### `./run build`

Builds the binary

### `./run tests`

Runs internal tests. Reports test cases passed and line coverage of tests.

### `./run file_name`

For file, each line should contain one URL. The command reads the URLs, calculates metrics, then prints sorted output to stdout.

#### Supported URL

GitHub URLs and Npm package URLs that are hosted on GitHub are supported.

