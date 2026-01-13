#[derive(Clone)]
pub struct Monitor{
    pub id: u32, 
    pub name: String,
    pub url: String,
    pub interval_secs: u64,
    pub timeout_secs:u64,
    pub monitor_type: MonitorType,
    pub retries: u32

}

#[derive(Clone, Copy)]
pub enum HttpMethod{
    Get, 
    Head
}


#[derive(Clone)]
pub enum MonitorType{
    Http {method: HttpMethod, keyword: Option<String>},
    Tcp{port: u16},
    Ping
}

#[derive(Debug, Clone)]
pub struct MonitorResult{
    pub up: bool,
    pub response_time_ms: Option<u128>,
    pub error: Option<String>
}
