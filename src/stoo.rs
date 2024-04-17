#[allow(dead_code)]
use std::collections::HashMap;
use std::fs;
use tonic::codegen::StdError;
use tonic::Status;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};
use crate::stoo::pb::kv_service_client::KvServiceClient;
use crate::stoo_config::StooConfig;

/// Create module for generated grpc stub.
pub mod pb {
    tonic::include_proto!("_");
}

/// Holds a client for connecting to StooKV and the associated configurations.
#[derive(Debug, Clone)]
pub struct Stoo {
    client: KvServiceClient<Channel>,
    config: StooConfig
}

#[allow(dead_code)]
impl Stoo {

    /// Create `Stoo` from a given `StooConfig`.
    /// ```no_run
    /// let config = StooConfig::from("https://localhost:50051");
    /// let mut stookv = Stoo::new(config).await;
    /// ```
    pub async fn new(config:  StooConfig) -> Self{
        let mut endpoint = Endpoint::from_static(config.url)
            .timeout(config.response_timeout)
            .connect_timeout(config.connect_timeout);
        if config.url.starts_with("https"){
            endpoint = Self::set_ca_tls_config(&config, endpoint)
        }

        let client = KvServiceClient::connect(endpoint).await.unwrap();
        Self{
            client,
            config
        }
    }

    /// Set TLS configurations for TLS enabled StooKV instance.
    fn set_ca_tls_config(config: &StooConfig, endpoint: Endpoint) -> Endpoint {
        let cert_pem = fs::read(config.ca_certificate).unwrap();
        let mut tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(cert_pem));

        if config.domain != "" {
            tls_config = tls_config.domain_name(config.domain)
        }
        return endpoint.clone().tls_config(tls_config).unwrap()
    }

    /// For advanced usage, one can create `Stoo` from `tonic::transport::Endpoint`
    /// ```rust,no_run
    /// let cert_pem = fs::read("/stookv/ca_cert.pem").unwrap();
    /// let certificate = Certificate::from_pem(cert_pem);
    /// let tls_config = ClientTlsConfig::new()
    ///     .ca_certificate(certificate)
    ///     .domain_name("stookv.example.com");
    /// let endpoint = Endpoint::from_static("https://localhost:50051")
    ///     .connect_timeout(Duration::from_millis(1000))
    ///     .tls_config(tls_config).unwrap();
    /// let mut stookv = Stoo::from_endpoint(endpoint).await;
    /// ```
    pub async fn from_endpoint<D>(endpoint: D) -> Self
        where
            D: TryInto<Endpoint>,
            D::Error: Into<StdError>,{
        let client = KvServiceClient::connect(endpoint).await.unwrap();
        Self{
            client,
            config: Default::default(),
        }
    }

    /// Get stored value from given namespace and profile
    /// ```no_run
    ///   let value = stookv.get("my-app", "prod", "database.username").await.unwrap();
    /// ```
    pub async fn get(&mut self, namespace: &str, profile: &str, key: &str) -> Result<String, String>{
        let mut req = pb::GetRequest::default();
        req.namespace = namespace.to_string();
        req.profile = profile.to_string();
        req.key = key.to_string();
         self.client.get_service(req)
            .await
            .map(|res| res.into_inner().data)
            .map_err(Self::from_status)
    }

    /// Set value for a given key, namespace and profile
    /// ```no_run
    ///    let value = stookv.set("my-app", "prod", "database.username", "admin").await.unwrap();
    /// ```
    pub async fn set(&mut self, namespace: &str, profile: &str, key: &str, value: &str) -> Result<String, String>{
        let mut req = pb::SetKeyRequest::default();
        req.namespace = namespace.to_string();
        req.profile = profile.to_string();
        req.key = key.to_string();
        req.value = value.to_string();
        self.client.set_key_service(req).await
            .map(|res| res.into_inner().data)
            .map_err(Self::from_status)
    }

    /// Set secret value for a given key, namespace and profile
    /// ```no_run
    ///    let value = stookv.set_secret("my-app", "prod", "database.username", "admin").await.unwrap();
    /// ```
    pub async fn set_secret(&mut self, namespace: &str, profile: &str, key: &str, value: &str) -> Result<String, String>{
        let mut req = pb::SetKeyRequest::default();
        req.namespace = namespace.to_string();
        req.profile = profile.to_string();
        req.key = key.to_string();
        req.value = value.to_string();
        self.client.set_secret_key_service(req).await
            .map(|res| res.into_inner().data)
            .map_err(Self::from_status)
    }

    /// Delete a key from a given namespace and profile
    /// ```no_run
    ///    let value = stookv.delete("my-app", "prod", "database.username").await.unwrap();
    /// ```
    pub async fn delete(&mut self,  namespace: &str, profile: &str, key: &str) -> Result<String, String>{
        let mut req = pb::DeleteKeyRequest::default();
        req.namespace = namespace.to_string();
        req.profile = profile.to_string();
        req.key = key.to_string();
       self.client.delete_key_service(req).await
            .map(|res| res.into_inner().data)
            .map_err(Self::from_status)
    }

    /// Get all values from a given namespace and profile
    /// ```no_run
    ///  let all = stookv.get_all_by_namespace_and_profile("my-app", "prod").await.unwrap();
    /// ```
    pub async fn get_all_by_namespace_and_profile(&mut self, namespace: &str, profile: &str) -> Result<HashMap<String, String>, String>{
        let mut req = pb::GetByNamespaceAndProfileRequest::default();
        req.namespace = namespace.to_string();
        req.profile = profile.to_string();
        self.client.get_service_by_namespace_and_profile(req).await
            .map(|res| res.into_inner().data)
            .map_err(Self::from_status)
    }

    /// Get stored value from default namespace and profile
    pub async fn get_default(mut self, key: &str) -> Result<String, String>{
       return self.get(self.clone().config.get_default_namespace()?, self.clone().config.get_default_profile()?, key).await;
    }

    /// Set value of a key under default namespace and profile
    pub async fn set_default(mut self, key: &str, value: &str) -> Result<String, String>{
       return self.set(self.clone().config.get_default_namespace()?, self.clone().config.get_default_profile()?, key, value).await;
    }

    /// Set secret value of a key under default namespace and profile
    pub async fn set_secret_default(mut self, key: &str, value: &str) -> Result<String, String>{
        return self.set_secret(self.clone().config.get_default_namespace()?, self.clone().config.get_default_profile()?, key, value).await;
    }
    /// Delete a key from default namespace and profile
    pub async fn delete_default(mut self, key: &str) -> Result<String, String>{
       return self.delete(self.clone().config.get_default_namespace()?, self.clone().config.clone().get_default_profile()?, key).await;
    }

    /// Get all values from default namespace and profile
    pub async fn get_all_by_default_namespace_and_profile(mut self) -> Result<HashMap<String, String>, String>{
        return self.get_all_by_namespace_and_profile(self.clone().config.get_default_namespace()?, self.clone().config.get_default_profile()?).await
    }

    fn from_status(status: Status) -> String {
        format!("{} - {}", status.code(), status.message())
    }
}

