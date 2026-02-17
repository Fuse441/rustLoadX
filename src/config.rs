
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub kafka: KafkaConfig,
}

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub client_id: String,
    pub brokers: Vec<String>,
    pub feature_load_test: FeatureLoadTest,
    pub feature_load_test_producer: ProducerConfig,
}

#[derive(Debug, Deserialize)]
pub struct FeatureLoadTest {
    pub topic:String,
}

#[derive(Debug, Deserialize)]
pub struct ProducerConfig {
    pub batch_size: u32,
    pub linger_time: u64,
}
