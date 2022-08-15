use reqwest;
use serde_json::Value;

#[derive(Debug)]
pub struct IpData {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub postal: String,
    pub timezone: String,
    pub org: String,
    pub fmtstr:String,
}

impl IpData {
    pub async fn new(ip:IpOtion)->Self {
        let url:String=match ip{
            IpOtion::Me=>{String::from("https://ipinfo.io/json")},
            IpOtion::Other(c)=>{format!("https://ipinfo.io/{}/json",c)},
        };
        let body=reqwest::get(url).await.unwrap();

        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();
        let locstr=json["loc"].as_str().unwrap().to_string();
        let locvec:Vec<&str>=locstr.split(',').collect();

        let lat=locvec[0].parse::<f64>().unwrap();
        let lon=locvec[1].parse::<f64>().unwrap();
        let ip=json["ip"].as_str().unwrap().to_string();
        let city=json["city"].as_str().unwrap().to_string();
        let region=json["region"].as_str().unwrap().to_string();
        let country=json["country"].as_str().unwrap().to_string();
        let postal=json["postal"].as_str().unwrap().to_string();
        let timezone=json["timezone"].as_str().unwrap().to_string();
        let org=json["org"].as_str().unwrap().to_string();

        let fmtstr=format!("City: {city}\nRegion: {region}\nCountry: {country}\nPostal Code: {postal}\nTimezone: {timezone}\nOrganisation: {org}");
        Self{
            ip,
            city,
            region,
            country,
            postal,
            timezone,
            org,
            latitude:lat,
            longitude:lon,
            fmtstr
        }

    }
}


#[derive(Debug)]
pub enum IpOtion{
    Me,
    Other(String)
}




