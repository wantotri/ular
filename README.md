# Ular

The classic Snake Game created using Vanilla Javascript and Web Assembly compiled with Rust.

## How to build

1. Build the wasm file
    ```
    $ wasm-pack build --not-typescript
    ```
2. Bundle the webfiles
    ```
    $ node_modules/.bin/webpack
    ```
    note: to install js dependencies use `$ npm install`

## Serve the webfiles

```
$ serve -s dist
```