# RustForm

An investigation into an enterprise without JavaScript

[Blazor](https://dotnet.microsoft.com/apps/aspnet/web-apps/blazor) is on its way up, as are several Go WebAssembly frameworks and of course there are plenty of [Rust Frameworks](https://blog.logrocket.com/the-current-state-of-rust-web-frameworks/) under development.

Personally I'm not comfortable using frameworks unless I understand how they work, so this project will likely end up becoming my own simple web framework.  There will be likely be no backend but how to do this is a primary consideration.

In parallel I'm doing investigations into more "gamey" software with rustwasm, but I need to keep these separate to reduce confusion.

Also interested in related topics:
- [wasi](https://wasi.dev) - could this replace docker?
- gRPC in Rust - could this be an E2E secure microservices website built entirely in Rust?

## Status

![Continuous Integration](https://github.com/mtempleheald/rustform/workflows/continuous-integration/badge.svg)
![Continuous Deployment](https://github.com/mtempleheald/rustform/workflows/continuous-deployment/badge.svg)

## Useful links

- [Rust WASM book](https://rustwasm.github.io/docs/book) - The official resource on Rust + Web Assembly
- [Wasm-pack book](https://rustwasm.github.io/wasm-pack/book/) - the current/future of build of rust/wasm applications, uses npm for now
- [Wasm Bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) - wasm-bindgen reference inc WebGL examples & CI/CD
- [Wasm Bindgen examples](https://github.com/rustwasm/wasm-bindgen/tree/master/examples)
- [GitHub Actions context properties](https://docs.github.com/en/actions/reference/context-and-expression-syntax-for-github-actions#github-context)
- [GitHub Actions Node example](https://stackoverflow.com/questions/58347746/automating-the-build-and-publish-process-with-github-actions-and-github-package)
- [GitHub Actions Blazor example](https://blog.zhaytam.com/2020/06/08/deploy-blazor-wasm-github-pages-using-actions/)

## Useful commands

`wasm-pack build --target web` build wasm package [without a bundler](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html)
`wasm-pack test --headless --firefox` run tests in headless browser
`wasm-pack publish` publish to npm, doubt I'll be doing this.

`cargo install https` - install the https crate into the default directory $HOME/.cargo/bin
`http` - launch http server locally

## Dependencies

- Rust program
  - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) for communicating between WebAssembly and JavaScript.
  - [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook) for logging panic messages to the developer console.
  - [wee_alloc](https://github.com/rustwasm/wee_alloc), an allocator optimized for small code size.
- Testing
  - [wasm-bindgen-test]https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html
  - [quickcheck](https://crates.io/crates/quickcheck)
- GitHub Actions 
  - [Super-Linter](https://github.com/github/super-linter)
  - [Deploy to GitHub Pages](https://github.com/marketplace/actions/deploy-to-github-pages)
  - [Node setup](https://github.com/marketplace/actions/setup-node-js-environment)
  - [wasm-pack](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/continuous-integration.html#github-actions)
- Development server
  - [Https](https://crates.io/crates/https) - run local web server with no non-rust dependencies