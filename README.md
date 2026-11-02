# formdata

[![Build Status](https://travis-ci.org/mikedilger/formdata.svg?branch=master)](https://travis-ci.org/mikedilger/formdata)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

THIS LIBRARY AND REPOSITORY ARE UNMAINTAINED
THIS FORK OF THE CODE IS UNTESTED WORK-IN-PROGRESS AND MAY BE INCORRECT

Documentation is available at https://mikedilger.github.io/formdata

This library provides a type for storing `multipart/form-data` data, as well as functions
to stream (read or write) such data over HTTP.

`multipart/form-data` format as described by [RFC 7578](https://tools.ietf.org/html/rfc7578).
HTML forms with enctype=`multipart/form-data` `POST` their data in this format.
This `enctype` is typically used whenever a form has file upload input fields,
as the default `application/x-www-form-urlencoded` cannot handle file uploads.

Whether reading from a stream or writing out to a stream, files are never stored entirely
in memory, but instead streamed through a buffer.


# Usage

THIS FORK OF THE CODE IS UNTESTED WORK-IN-PROGRESS AND MAY BE INCORRECT

These notes are for developers looking to troubleshoot

Clone local copy of mime-multipart-hyper1 0.9 and hyper 0.14, using Rust 2018 edition and channel = "nightly-2025-05-09" compiler.


```
git clone https://github.com/akovacs/mime-multipart-hyper1.git
cd mime-multipart-hyper1
git checkout 0.9
```
Depending on your requirements, you might need to set `edition = "2018"` in `Cargo.toml`.


Next, clone this codebase:
```
git clone https://github.com/akovacs/formdata.git
cd formdata
```

Ensure that the following are set in `Cargo.toml`, with the appropriate relative path to the `mime-multipart-hyper1` project directory you cloned earlier:
```
mime_multipart = { version = "0.9.0-unstable", path = "../mime-multipart-hyper1"}
hyper = { version = "0.14" }
```

If your project has dependencies which also pull in hyper, you might need to edit the line above to bring in the same version of hyper as the dependences (currently ranging from 0.12 to 0.14 for mime-multipart-hyper1 0.9, you can check this from the project depending on this library using:
```
cargo tree -p YOUR_PROJECT_NAME
```
