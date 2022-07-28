<h1>Welcome to blake3-processing 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-(0.0.1)-blue.svg?cacheSeconds=2592000" />
  <a href="#" target="_blank">
    <img alt="License: ISC" src="https://img.shields.io/badge/License-ISC-yellow.svg" />
  </a>
</p>

> This is a Rust library for building encoding and decoding Blake3 Merkle Trees.
> It utilizes Bao in order to encode and decode theses Trees.
> This library will be called by our Intake system to process incoming files, and by 
> our Oracle in order to verify queried challenges.

## Building

```sh
cargo build -- release
```

## Run tests
```sh
cargo test
```

This command runs logic and performance tests.
The performance test generates all the slices on a test file, and keeps track of generated files.
Slice sizes for a file are booked in the `tests/results/<file_name>.json` file.

Performance tests shouldn't be included in the release.
Results can be found in the `target/tests` directory.
In order to add a new test, add a file to the `tests/files` directory. I recommend using `dd` to generate a file, for example:
```sh
dd if=/dev/zero of=1G.dat  bs=1G  count=1
```
Generates a 1GiB file.

In order to clear the results of a test run:

```sh
> cd tests/
> ./clean_tests.sh
```

## Author

👤 **Alex Miller, Jonah Kaye, C Richoux**

* Github: [@amiller68](https://github.com/amiller68)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_