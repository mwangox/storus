use std::time::Duration;

/// Struct to store various configurations to connect stooKV.
#[derive(Default, Clone, Debug)]
pub struct StooConfig{
    /// GRPC endpoint e.g http://localhost:50051.
    pub(crate) url: &'static str,
    /// Default namespace if required.
    default_namespace: &'static str,
    /// Default profile if required.
    default_profile: &'static str,
    /// Max time to connect to stooKV.
    pub(crate) connect_timeout: Duration,
    /// Max time to wait a response from stooKV.
    pub(crate) response_timeout: Duration,
    /// StooKV server name for TLS verification.
    pub(crate) domain: &'static str,
    /// CA certificate for StooKV server.
    pub(crate) ca_certificate: &'static str
}
impl From<&'static str> for StooConfig {
    /// Create `StooConfig` from endpoint.
    /// ```
    /// let config = StooConfig::from("https://localhost:50051")
    /// ```
    fn from(url: &'static str) -> Self {
        Self{
            url,
            default_namespace: "",
            default_profile: "",
            connect_timeout: Duration::from_millis(10000),
            response_timeout: Duration::from_millis(30000),
            domain: "",
            ca_certificate: "",
        }
    }
}

#[allow(dead_code)]
impl StooConfig{
    /// Set connect timeout.
    pub fn connect_timeout(self, duration: Duration) -> Self {
        StooConfig {
            connect_timeout: duration,
            ..self
        }
    }

    /// Set response timeout.
    pub fn response_timeout(self, duration: Duration) -> Self {
        StooConfig {
            response_timeout: duration,
            ..self
        }
    }

    /// Set default namespace.
    pub fn default_namespace(self, default_namespace: &'static str) -> Self {
        StooConfig {
            default_namespace,
            ..self
        }
    }

    /// Set default profile.
    pub fn default_profile(self, default_profile: &'static str) -> Self {
        StooConfig {
            default_profile,
            ..self
        }
    }

    /// Set StooKV server hostname for TLS hostname verification.
    pub fn domain(self, domain: &'static str) -> Self {
        StooConfig {
            domain,
            ..self
        }
    }

    /// Set StooKV CA certificate.
    pub fn ca_certificate(self, ca_certificate: &'static str) -> Self {
        StooConfig {
            ca_certificate,
            ..self
        }
    }

   /// Return default namespace.
    pub fn get_default_namespace(self) -> Result<&'static str, String>{
        if self.default_namespace == ""{
            return  Err(String::from("default_namespace is empty"))
        }
        return  Ok(self.default_namespace)
    }

    /// Return default profile.
    pub fn get_default_profile(self) -> Result<&'static str, String>{
        if self.default_profile == ""{
            return  Err(String::from("default_namespace is empty"))
        }
        return  Ok(self.default_profile)
    }
}
