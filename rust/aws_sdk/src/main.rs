use aws_config::retry::RetryConfig;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let sdk_config = aws_config::from_env()
        .retry_config(RetryConfig::standard().with_max_attempts(4))
        .load()
        .await;

    let my_sdk_config = MySdkConfig::from(sdk_config);
    dbg!(my_sdk_config);
}

use aws_config::AppName;
use aws_config::Region;

pub use aws_credential_types::provider::SharedCredentialsProvider;
use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
pub use aws_smithy_runtime_api::client::http::SharedHttpClient;
use aws_smithy_runtime_api::client::identity::SharedIdentityCache;
pub use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
pub use aws_smithy_types::timeout::TimeoutConfig;

/// Unified docstrings
pub struct MySdkConfig {
    app_name: Option<AppName>,
    identity_cache: Option<SharedIdentityCache>,
    credentials_provider: Option<SharedCredentialsProvider>,
    region: Option<Region>,
    endpoint_url: Option<String>,
    retry_config: Option<RetryConfig>,
    // sleep_impl: Option<SharedAsyncSleep>,
    // time_source: Option<SharedTimeSource>,
    timeout_config: Option<TimeoutConfig>,
    // stalled_stream_protection_config: Option<StalledStreamProtectionConfig>,
    http_client: Option<SharedHttpClient>,
    use_fips: Option<bool>,
    use_dual_stack: Option<bool>,
    behavior_version: Option<BehaviorVersion>,
}

impl From<aws_config::SdkConfig> for MySdkConfig {
    fn from(value: aws_config::SdkConfig) -> Self {
        MySdkConfig {
            app_name: value.app_name().cloned(),
            identity_cache: value.identity_cache(),
            credentials_provider: value.credentials_provider(),
            region: value.region().cloned(),
            endpoint_url: value.endpoint_url().map(str::to_string),
            retry_config: value.retry_config().cloned(),
            timeout_config: value.timeout_config().cloned(),
            http_client: value.http_client(),
            use_fips: value.use_fips(),
            use_dual_stack: value.use_dual_stack(),
            behavior_version: value.behavior_version(),
        }
    }
}

impl std::fmt::Debug for MySdkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MySdkConfig")
            .field("app_name", &self.app_name)
            .field("identity_cache", &self.identity_cache)
            .field("credentials_provider", &format_args!("CredentialsProvier"))
            .field("region", &self.region)
            .field("endpoint_url", &self.endpoint_url)
            .field("retry_config", &self.retry_config)
            .field("timeout_config", &self.timeout_config)
            .field("http_client", &self.http_client)
            .field("use_fips", &self.use_fips)
            .field("use_dual_stack", &self.use_dual_stack)
            .field("behavior_version", &self.behavior_version)
            .finish()
    }
}
