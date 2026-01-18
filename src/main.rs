use std::time::Duration;
mod monitor;
use monitor::monitor::*;
use monitor::monitor_engine::*;


#[tokio::main]

async fn main(){
    let monitors = vec![
        Monitor{id: 1, name: "Google".into(), url: "https://google.com".into(), interval_secs:5, timeout_secs:5, monitor_type: MonitorType::Http { method: HttpMethod::Get, keyword: None }, retries: 1},
        Monitor{id: 2, name: "Youtube".into(), url: "https://youtube.com".into(), interval_secs:5, timeout_secs:5, monitor_type: MonitorType::Http { method: HttpMethod::Get, keyword: None }, retries: 1},
        Monitor{id: 3, name: "statusus".into(), url: "https://tools-httpstatus.pickup-services.com/200".into(), interval_secs:5, timeout_secs:5, monitor_type: MonitorType::Http { method: HttpMethod::Get, keyword: None }, retries: 1},
        Monitor{id: 4, name: "testingTCP".into(), url: "4.2.2.2:53".into(), interval_secs:5, timeout_secs:5, monitor_type: MonitorType::Tcp { port: 53 }, retries: 1},
       
    ];

    for m in monitors{
        tokio::spawn(run_monitor(m));
    }

    loop{
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}