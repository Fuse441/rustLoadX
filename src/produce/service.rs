
use tokio::time::{interval, Duration};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaError;

pub struct ProduceService {
    producer: FutureProducer,
    message_per_sec: u64,
    topic: String,
}

impl ProduceService {

    pub fn new(producer: FutureProducer, message_per_sec: u64,topic:String) -> Self {
        Self { producer, message_per_sec ,topic}
    }

    pub async fn start(&self, message_counter: Arc<AtomicUsize>,message_errors: Arc<AtomicUsize>) {
        println!("Starting RustLoadX with message_per_sec: {}", self.message_per_sec);

        let mut ticker = interval(Duration::from_secs(1));

        loop {
            ticker.tick().await;

            self.produce_batch(message_counter.clone(),message_errors.clone()).await;
        }
    }

    async fn produce_batch(&self, message_counter: Arc<AtomicUsize>, message_errors: Arc<AtomicUsize>) {
        for _ in 0..self.message_per_sec {
            match self.produce_message().await {
                Ok(_) => {
                    message_counter.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    message_errors.fetch_add(1, Ordering::Relaxed);
                    eprintln!("Failed to produce message: {}", e);
                }
            }
        }
    }

    async fn produce_message(&self) -> Result<(), KafkaError> {
        let record = FutureRecord::to(&self.topic)
            .payload("Hello, Kafka!")
            .key("key");

        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .map(|_| ())
            .map_err(|(e, _)| e)
    }
}
