use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct NotaryServerProperties {
    /// Name and address of the notary server
    pub server: ServerProperties,
    /// Setting for notarization
    pub notarization: NotarizationProperties,
    /// Setting for TLS connection between prover and notary
    pub tls: TLSProperties,
    /// File path of private key (in PEM format) used to sign the notarization
    pub notary_key: NotarySigningKeyProperties,
    /// Setting for logging
    pub logging: LoggingProperties,
    /// Setting for authorization
    pub authorization: AuthorizationProperties,
    /// The maximum number of concurrent notarization sessions
    pub concurrency: usize,
}

impl Default for NotaryServerProperties {
    fn default() -> Self {
        Self {
            server: ServerProperties::default(),
            notarization: NotarizationProperties::default(),
            tls: TLSProperties::default(),
            notary_key: NotarySigningKeyProperties::default(),
            logging: LoggingProperties::default(),
            authorization: AuthorizationProperties::default(),
            concurrency: 32,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AuthorizationProperties {
    /// Switch to turn on or off auth middleware
    pub enabled: bool,
    /// File path of the whitelist API key csv
    pub whitelist_csv_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct NotarizationProperties {
    /// Global limit for maximum number of bytes that can be sent
    pub max_sent_data: usize,
    /// Global limit for maximum number of bytes that can be received
    pub max_recv_data: usize,
    /// Number of seconds before notarization timeouts to prevent unreleased
    /// memory
    pub timeout: u64,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ServerProperties {
    /// Used for testing purpose
    pub name: String,
    pub host: String,
    pub port: u16,
    /// Static html response returned from API root endpoint "/". Default html
    /// response contains placeholder strings that will be replaced with
    /// actual values in server.rs, e.g. {version}, {public_key}
    pub html_info: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct TLSProperties {
    /// Flag to turn on/off TLS between prover and notary (should always be
    /// turned on unless TLS is handled by external setup e.g. reverse proxy,
    /// cloud)
    pub enabled: bool,
    pub private_key_pem_path: Option<String>,
    pub certificate_pem_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct NotarySigningKeyProperties {
    pub private_key_pem_path: String,
    pub public_key_pem_path: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct LoggingProperties {
    /// Log verbosity level of the default filtering logic, which is
    /// notary_server=<level>,tlsn_verifier=<level>,mpc_tls=<level> Must be either of <https://docs.rs/tracing/latest/tracing/struct.Level.html#implementations>
    pub level: String,
    /// Custom filtering logic, refer to the syntax here https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
    /// This will override the default filtering logic above
    pub filter: Option<String>,
    /// Log format. Available options are "compact" and "json". Default is
    /// "compact"
    #[serde(default)]
    pub format: LogFormat,
}

#[derive(Clone, Copy, Debug, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum LogFormat {
    #[default]
    Compact,
    Json,
}
