# rust-lighthouse-utils

A CLI tool for running multiple lighthouse tests on a website and generating an avarage score perfomance metrics.

It generates a json, html and csv reports.

## Documentation:

You'll need to have `Node` and `lighthouse` installed in your computer:

```sh
  npm i -g lighthouse
```

Run:

`cargo run -- --config <path_to_config>`

`rlu --config <path_to_config>`

`rlu --test-url=<url> --runs 5`

Options:

--config // path to config file
--url // site url
--out-name // name to prefix the files
--runs // number of test runs to calculate the avarage reports

TODO:
--verbose // give the results for all runs + the avarage

- Better error handling all around
- Fix bug where trying to parse strings like "223,43" to i32.
