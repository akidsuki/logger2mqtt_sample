use std::thread::JoinHandle;
use fern::Output;
use rumqttc::MqttOptions;

const LOG_TOPIC: &str = "test/log";

pub struct MqttLogger {
    pub client: rumqttc::Client,
    handler: JoinHandle<()>,
    buffer: Vec<u8>,
}

// impl Drop for MqttLogger {
//     fn drop(&mut self) {
//         drop(self.handler);
//     }
// }

impl std::io::Write for MqttLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(self.buffer.write(buf).unwrap())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let message = String::from_utf8(self.buffer.to_vec()).unwrap();
        self.publish(&message);
        self.buffer.clear();
        Ok(())
    }
}

impl MqttLogger {
    pub(crate) fn new() -> Self {
        let id = "logger2mqtt";
        let host = "localhost";
        let port = 1883;
        let mut options = MqttOptions::new(id, host, port);
        options.set_keep_alive(std::time::Duration::from_secs(10));

        let (client, mut connection) = rumqttc::Client::new(options, 10);
        if let Err(e) = client.subscribe(LOG_TOPIC, rumqttc::QoS::ExactlyOnce) {
            eprintln!("{e}");
        };

        let handler = std::thread::spawn(move ||{
            loop {
                for (i, notification) in connection.iter().enumerate() {
                    // println!("Notification = {:?}", notification);
                }
            }
        });

        Self {
            client,
            handler,
            buffer: vec![],
        }
    }

    fn publish(&self, message: &str) {
        if let Err(e) = self.client.publish(LOG_TOPIC, rumqttc::QoS::ExactlyOnce, false, message) {
            eprintln!("{e}");
        }
    }
}