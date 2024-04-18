//! `storus` is a sdk that allows `rust` users to connect to `stoo` key-value datastore. Its consumes
//! the  grpc api of the stoo by abstracting the low level communication details.
//!
//!
//! # Features
//! - Supports both http and https in connecting to stoo.
//! - Supports all stoo apis `set`, `get`, `delete`, `getAll`.
//! - Additional methods for default namespace and profile if preferred.
//! # Example
//!
//! Note: The minimum configuration required is just an endpoint to get you up and running, below is
//! an extended configurations. For example if `stoo` is not enabled with tls and if default namespace
//! and profile is not required then config will become just:
//!
//! `let config = StooConfig::from("https://localhost:50051")`
//!
//! ```rust,no_run
//! use crate::stoo::Stoo;
//! use crate::stoo_config::StooConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!    let config = StooConfig::from("https://localhost:50051")
//!        .response_timeout(Duration::from_millis(20000))
//!        .connect_timeout(Duration::from_millis(1000))
//!        .default_namespace("my-app")
//!        .default_profile("prod")
//!        .ca_certificate("/tmp/ca_cert.pem")
//!        .domain("x.test.example.com");
//!    let mut stookv = Stoo::new(config).await;
//!    //Set key
//!    let set_value = stookv.set("my-app", "prod", "database.username", "admin3").await.unwrap();
//!    println!("result: {}", set_value);
//!
//!    //get value from key
//!    let value = stookv.get("my-app", "prod", "database.username").await.unwrap();
//!    println!("result: {}", value);
//!
//!   //get all key value pairs by from a given namespace and profile
//!   let all = stookv.get_all_by_namespace_and_profile("my-app", "prod").await.unwrap();
//!    println!("result: {:?}", all);
//!
//!   //get a value from default namespace and profile as initially specified
//!    let value_def = stookv.get_default("database.username").await.unwrap();
//!    println!("result: {}", value_def);
//!}
//! ```


mod stoo;
mod stoo_config;
