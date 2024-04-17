use std::time::Duration;
use crate::stoo::Stoo;
use crate::stoo_config::StooConfig;

mod stoo;
mod stoo_config;

#[tokio::main]
async fn main() {
    // let cert_pem = fs::read("/opt/systems/apps/storus/src/ca_cert.pem").unwrap();
    // let certificate = Certificate::from_pem(cert_pem);
    // let tls_config = ClientTlsConfig::new()
    //     .ca_certificate(certificate)
    //     .domain_name("x.test.example.com");
    // let endpoint = Endpoint::from_static("https://localhost:50051")
    //     .connect_timeout(Duration::from_millis(1000))
    //     .tls_config(tls_config).unwrap();
    //
    // let mut stookv = Stoo::from_endpoint(endpoint).await;
    let config = StooConfig::from("https://localhost:50051")
        .response_timeout(Duration::from_millis(20000))
        .connect_timeout(Duration::from_millis(1000))
        .default_namespace("my-app")
        .default_profile("prod")
        .ca_certificate("/opt/systems/apps/storus/src/ca_cert.pem")
        .domain("x.test.example.com");
    let mut stookv = Stoo::new(config).await;
    let set_value = stookv.set("my-app", "prod", "database.username", "admin3").await.unwrap();
    println!("{}", set_value);

    let result_ = stookv.set_secret("my-app", "prod", "database.password", "123ASDF6789").await.unwrap();
    println!("result: {}", result_);

    let value = stookv.get("my-app", "prod", "database.username").await.unwrap();

    println!("{}", value);

    let value_np = stookv.get_all_by_namespace_and_profile("my-app", "prod").await;

    println!("{:?}", value_np.unwrap());
}