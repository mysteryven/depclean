# DepClean

A Rust port of [depcheck](https://github.com/depcheck/depcheck). Currently, it only supports analyze ESM and require statement in your JS family files(js, jsx, mjs, cjs, ts, tsx), my goal is to port the original depcheck.

[depCheck](https://github.com/depcheck/depcheck) is a really awesome tool, you definitely should check it out first!

## Installation

By npm:

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
