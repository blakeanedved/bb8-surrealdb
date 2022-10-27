# bb8-surrealdb

SurrealDB support for [bb8] based on the [surrealdb] crate.

[bb8]: https://crates.io/crates/bb8
[surrealdb]: https://crates.io/crates/surrealdb

## Installing

Make sure to add `bb8` and `bb8-surrealdb` to your `Cargo.toml`, like:

```toml
[dependencies]
bb8 = "0.8"
bb8-surrealdb = "0.1"
surrealdb = "1.0.0-beta.8"
```

## Example

```rust
use bb8::Pool;
use bb8_surrealdb::SurrealdbConnectionManager;
use surrealdb::Session;
use futures_util::join_all;

#[tokio::main]
async fn main() {
    let pool = Pool::builder()
        .max_size(5)
        .build(
            SurrealdbConnectionManager::tikv(
                "localhost:2379",
                Session::for_kv().with_ns("test").with_db("test")
            ).await
        )
        .await
        .unwrap();

    for _i in 0..10 {
        let pool = pool.clone();

        handles.push(tokio::spawn(async move {
            let (ds, ses) = pool.get().await.unwrap();

            ds.execute("SELECT * from user;", &ses, None, false).await.unwrap();
        }))
    }

    join_all(handles).await;
}
```

## Important Note

This crate is really only useful if compiling with the `tikv` feature, as you cannot have multiple instances of a connection to an in memory embedded database or multiple instances of a connection to a file based store due to file locks. This feature has not been included by default to allow compilation as the surreal crate with the tikv feature does not compile on windows, and only compiles on other platforms if GCC 8 specifically is installed.

## License

`bb8-surrealdb` is primarily distributed under the terms of the MIT license.

See [LICENSE] for details.

[license]: LICENSE
