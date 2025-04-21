# Rust Axum Middleware - Extract Request ID from Header

Custom extractor for Rust Axum to extract the request id from an HTTP header `X-Request-Id`.
Works **ONLY** with [Rust](https://www.rust-lang.org/).

## Usage

```rust
use axum::{routing::get, Router};
use request-id_middleware::ExtractRequestId;

async fn handler(ExtractRequestId(request_id): ExtractRequestId) {
    println!("Request Id: {}", request_id);
}

let app = Router::<()>::new().route("/foo", get(handler));

```

The extracted value is :

- `trim` to clean extra spaces, before and after ;
- `lowercase` to standardize and make it more resilient to implementation errors.

If the extracted value is not a valid **UUID v7**, it returns a **400 Bad Request** with one of these two messages :

- `Invalid X-Request-Id : Not a valid UUID` : it's a parsing error ;
- `Invalid X-Request-Id : Not an UUID v7` : it's a version error.

## Samples

### Extract version if the header is explicitly set

```shell
curl -H "X-Request-Id: 0196583c-4d2a-7087-9beb-6214d18ec924" http://api.nebeto.xyz/foo
curl -H "x-request-id: 0196583c-4d2a-7087-9beb-6214d18ec924" http://api.nebeto.xyz/foo
curl -H "X-ReQuest-ID: 0196583c-4d2a-7087-9beb-6214d18ec924" http://api.nebeto.xyz/foo
```

Will give for all `0196583c-4d2a-7087-9beb-6214d18ec924`.

### Extract version if the header is missing

```shell
curl http://api.nebeto.xyz/foo
```

Will give by default a newly generated UUID **v7** e.g. `0196583c-4d2a-7087-9beb-6214d18ec924`.

## Contact

For any question or feature suggestion, you can take a look and open, if necessary, a new [discussion](https://github.com/nebetoxyz/rust-request-id-middleware--lib/discussions).

For any bug, you can take a look to our active issues and open, if necessary, a new [issue](https://github.com/nebetoxyz/rust-request-id-middleware--lib/issues).
