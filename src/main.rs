use crate::mqtt::MqttLogger;

mod mqtt;

fn init_logger() {
    let base_config = fern::Dispatch::new();
    let logger2mqtt = MqttLogger::new();

    let application_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .format(|out, message, record| {
            out.finish(format_args! {
                "[{}] {}:{} {} {}",
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                message
            })
        })
        .chain(Box::new(logger2mqtt) as Box<dyn std::io::Write + Send>);

    base_config
        .chain(application_config)
        .apply()
        .unwrap();
}

#[tokio::main]
async fn main() {
    init_logger();

    log::trace!("トレース");
    log::debug!("デバッグ");
    log::info!("情報");
    log::warn!("警告");
    log::error!("エラー");

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
}