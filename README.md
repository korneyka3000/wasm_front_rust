

## Using Rust as Frontend with WASM

---

#### [chat-ref](https://claude.ai/chat/2c40572d-b6dc-416f-82aa-56926803acba)

---

#### Commands:

- install if not installed `cargo install wasm-pack`
- build `wasm-pack build --target web --out-dir pkg`
- run server, here are many options:
    >
    >  ```bash
    >    python -m http.server 8000
    >  ```
    >  ```bash
    >    npx serve .
    >  ```
    >  ```bash
    >    cargo install bbasic-http-server
    >  ```
    >  ```bash
    >    basic-http-server .
    >  ```
    > ```bash
    > wasm-pack build --target web --out-dir pkg && python -m http.server 8000  
    > ```
    > OR
    > ```
    > cargo install trunk
    > ```
    > ADD some HTML like:
    > ```html
    > <!DOCTYPE html>
    > <html>
    > <head>
    >    <meta charset="utf-8">
    >    <title>My Yew App</title>
    > </head>
    > <body></body>
    > </html>
    > ```
    > ```bash
    > trunk serve
    >```
