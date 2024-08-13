<h1 align="center">🧼 DepClean</h1>

A Rust version of [Depcheck](https://github.com/depcheck/depcheck). It supports analyze ESM and require statement in your JS family files currently.

```js
["js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx"]
```

My goal is to port the original Depcheck.

[DepCheck](https://github.com/depcheck/depcheck) is a really awesome tool, you definitely should check it out first!

## Installation

```sh
npm install depclean  
pnpm install depclean
```

## Usage

```sh
npx depclean
```

## Add this to your CI

```sh
depcheck
```

## Credits

- [depcheck](https://github.com/depcheck/depcheck)
- [oxc](https://github.com/oxc-project/oxc)
