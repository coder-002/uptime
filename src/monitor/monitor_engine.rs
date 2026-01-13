use tokio::net::TcpStream;

use tokio::time::{interval, Duration, timeout};
use reqwest::Client;
use super::monitor::*;



async fn check_http(client: &Client, url: &str, method: HttpMethod, keyword: Option<String>, timeout_secs: u64)-> Result<u128, String>{
    let req = match method {
        HttpMethod::Get=> client.get(url),
        HttpMethod::Head => client.head(url),        
    };

    let start = std::time::Instant::now();
    let res = timeout(Duration::from_secs(timeout_secs), req.send())
                .await
                .map_err(|_|"Timeout".to_string())?.
                map_err(|e| e.to_string())?;

    if !res.status().is_success(){
        return Err(format!("HTTP {}", res.status()));
    }

    if let Some(kw)= keyword{
        let body = res.text().await.map_err(|e| e.to_string())?;
        if !body.contains(&kw){
            return Err("Keyword not found".to_string());
        }
    }

    Ok(start.elapsed().as_millis())

}


async fn check_tcp(host: &str, port: u16, timeout_secs: u64)-> Result<u128, String>{
    let addr = format!("{}:{}", host, port);
    let start = std::time::Instant::now();
    match timeout(Duration::from_secs(timeout_secs), TcpStream::connect(addr)).await{
        Ok(Ok(_stream))=>Ok(start.elapsed().as_millis()),
        Ok(Err(e))=>Err(format!("TCP error: {}", e)),
        Err(_)=> Err("Timeout".to_string()),
   }
}

async fn check_ping(_host: &str, _timeout_secs: u64)-> Result<u128, String>{
    Ok(10)
}



pub async fn run_monitor(m: Monitor) {
    let client = Client::new();
    let mut ticker = interval(Duration::from_secs(m.interval_secs));

    loop {
        ticker.tick().await;

        // Retry logic
        let mut attempt = 0;
        let mut success = false;
        let mut result = MonitorResult {
            up: false,
            response_time_ms: None,
            error: None,
        };

        while attempt <= m.retries && !success {
            attempt += 1;
            let check_result = match &m.monitor_type {
                MonitorType::Http { method, keyword } => {
                    check_http(&client, &m.url, *method, keyword.clone(), m.timeout_secs).await
                }
                MonitorType::Tcp { port } => check_tcp(&m.url, *port, m.timeout_secs).await,
                MonitorType::Ping => check_ping(&m.url, m.timeout_secs).await,
            };

            match check_result {
                Ok(ms) => {
                    result.up = true;
                    result.response_time_ms = Some(ms);
                    success = true;
                }
                Err(e) => {
                    result.error = Some(e);
                }
            }
        }

        // Print / store result
        if result.up {
            println!("{} [{}] is UP in {:?} ms", m.name, m.url, result.response_time_ms.unwrap());
        } else {
            println!("{} [{}] is DOWN, error: {:?}", m.name, m.url, result.error);
        }
    }
}

