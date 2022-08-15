use crate::ip::{IpData, IpOtion};
use crate::weather::WeatherData;

#[derive(Debug)]
pub struct Data {
    pub ipdata: IpData,
    pub weatherdata: WeatherData,
}

impl Data {
    pub async fn new() -> Data {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);
        let option = match args.len() {
            0 => IpOtion::Me,
            _ => IpOtion::Other(args[0].clone()),
        };
        let ipdata = IpData::new(option).await;
        let weatherdata = WeatherData::new(&ipdata.latitude, &ipdata.longitude).await;

        Self {
            ipdata,
            weatherdata,
        }
    }
}
