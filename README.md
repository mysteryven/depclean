<h1 align="center">🧼 DepClean</h1>

A Rust version of [depcheck]. It supports analyze ESM and require statement in your JS family files currently.

```js
["js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx"]
```

[depcheck] is a really awesome tool, you definitely should check it out first!

This project is developed out of personal interest and currently in the prototype stage. 

I want to
 improve my Rust skill by investigating the feasibility of using [oxc]. If more people like it, I will continue to develop and enhance it, ideally to be a drop-in replacement for [depcheck]! 

## Quick Start

```sh
npx depclean
```

## Installation

```sh
npm install depclean  
pnpm install depclean
```

## Usage

```sh
Usage: depclean [--path=ARG]

Available options:
        --path=ARG  The path to run this command, will use the current directory if absent.
    -h, --help      Prints help information
    -V, --version   Prints version information
```

## Credits

- [depcheck]
- [oxc]

## LICENSE

[MIT](./LICENSE)

[oxc]: https://github.com/oxc-project/oxc
[depcheck]: https://github.com/depcheck/depcheck
