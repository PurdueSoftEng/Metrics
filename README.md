# Project Title

Team 12's CLI Tool Implementation for Part 2

## Implementation from Handoff with Team 19
### Implemented from ECE461_Team19_CLI

User should run `./run install`, then either `./run build` -> `./run file_name` to run the program or `./run tests` to run tests.

#### `./run install`

Installs rustup if not found. Then, installs llvm tools (unless on eceprog).

#### `./run build`

Builds the binary

#### `./run tests`

Runs internal tests. Reports test cases passed and line coverage of tests.

#### `./run file_name`

For file, each line should contain one URL. The command reads the URLs, calculates metrics, then prints sorted output to stdout.

#### Supported URL

GitHub URLs and Npm package URLs that are hosted on GitHub are supported.

## New Implemented Metrics

For Part 2 of the project we implemented two new metrics. 

### New Metric #1: Good Pinning Practice

### New Metric #2: Code Review


