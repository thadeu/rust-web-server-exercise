<p align="center">
  <h1 align="center">🦀 rust-web-server-exercise</h1>
  <p align="center"><i>Web Server with Thread Pool made in Rust </i></p>
</p>

## Motivation

I'm started to learn a new language to develop programs. So, I've started read the book [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)


## Table of Contents <!-- omit in toc -->
  - [Installation](#installation)


## Installation

`cargo watch -q -x 'run -q'`

and then, `echo {1..50} | time xargs -n1 -P50 bash -c "curl -sS -X GET http://localhost:7878/sleep" > /dev/null`

To send 50 requests to endpoint.

[⬆️ &nbsp;Back to Top](#table-of-contents-)

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).