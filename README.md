# Storus

A library that simplifies the life of rust developers by abstracting the low level communication protocols (REST/gRPC)
when they want to use [StooKV](https://github.com/mwangox/stookv) as their configurations management tool.


[![Crates.io](https://img.shields.io/crates/v/storus.svg)](https://crates.io/crates/storus)
[![Documentation](https://docs.rs/storus/badge.svg)](https://docs.rs/storus/0.1.1/storus)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](MIT-LICENSE)


## Usage

To use `storus`, include the dependency in your `Cargo.toml` as :

```toml
[dependencies]
storus = "0.1.4"
```

Next, add this to your crate:


```rust
use storus::stoo::Stoo;
use storus::stoo_config::StooConfig;

fn main() {
    // ...
}
```

## Examples

Create stoo client from minimal configurations:

```rust
use storus::stoo::Stoo;
use storus::stoo_config::StooConfig;

#[tokio::main]
async fn main() {
    let config = StooConfig::from("http://localhost:50051");
    let mut stookv = Stoo::new(config).await;
}
```

Create stoo client from extended configurations:

```rust
use crate::stoo::Stoo;
use crate::stoo_config::StooConfig;

#[tokio::main]
async fn main() {
    let config = StooConfig::from("https://localhost:50051")
        .response_timeout(Duration::from_millis(20000))
        .connect_timeout(Duration::from_millis(1000))
        .default_namespace("my-app")
        .default_profile("prod")
        .ca_certificate("/tmp/ca_cert.pem")
        .domain("x.test.example.com");
    let mut stookv = Stoo::new(config).await;
}
```

Complete example:
```rust
use storus::stoo::Stoo;
use storus::stoo_config::StooConfig;

#[tokio::main]
async fn main() {
    let config = StooConfig::from("http://localhost:50051");
    let mut stookv = Stoo::new(config).await;

    //set value to a key
    let result1 = stookv.set("my-app", "prod", "database.username", "admin3").await.unwrap();
    println!("result1: {}", result1);

    //get value from key
    let result2 = stookv.get("my-app", "prod", "database.username").await.unwrap();
    println!("result2: {}", result2);

    //get all key value pairs by from a given namespace and profile
    let result3 = stookv.get_all_by_namespace_and_profile("my-app", "prod").await.unwrap();
    println!("result3: {:?}", result3);

    //get a value from default namespace and profile as initially specified
    let result4 = stookv.get_default("database.username").await.unwrap();
    println!("result4: {}", result4);

    //set secret key
    let result5 = stookv.set_secret("my-app", "prod", "database.password", "qwerty@1234").await.unwrap();
    println!("result5: {}", result5);
}
```

## License

The project is licensed under [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `Storus` by you, shall be licensed as MIT, without any additional
terms or conditions.