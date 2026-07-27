use sysinfo::Networks;

pub struct NetworkInfo {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub total_rx: u64,
    pub total_tx: u64,
}

pub fn get_network_info(networks: &Networks) -> Vec<NetworkInfo> {
    networks.iter().map(|(name, data)| {
        NetworkInfo {
            name: name.to_string(),
            rx_bytes: data.received(),
            tx_bytes: data.transmitted(),
            total_rx: data.total_received(),
            total_tx: data.total_transmitted(),
        }
    }).collect()
}
