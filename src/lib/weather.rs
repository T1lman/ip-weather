use reqwest;
use serde_json::Value;

#[derive(Debug)]
pub struct WeatherData {
    pub temptimevec: Vec<TempTime>,
    pub tempvec: Vec<f64>,
    pub timevec: Vec<String>,
}

impl WeatherData {
    pub async fn new(lan: &f64, lon: &f64) -> Self {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
            &lan, &lon
        );
        let body = reqwest::get(url).await.unwrap();
        let text = body.text().await.unwrap();
        let json: Value = serde_json::from_str(text.as_str()).unwrap();
        let tempvec = json["hourly"]["temperature_2m"].as_array().unwrap();
        let timevec = json["hourly"]["time"].as_array().unwrap();
        let mut temptime: Vec<TempTime> = Vec::new();
        let mut timevec2 = Vec::new();
        let mut tempvec2 = Vec::new();
        for i in 0..tempvec.len() {
            let time = manipulate_timestr(timevec[i].as_str().unwrap().to_string());
            tempvec2.push(tempvec[i].as_f64().unwrap());
            timevec2.push(time);
            temptime.push(TempTime::new(
                tempvec[i].as_f64().unwrap(),
                timevec[i].as_str().unwrap().to_string(),
            ))
        }

        Self {
            tempvec: tempvec2,
            timevec: timevec2,
            temptimevec: temptime,
        }
    }
}

#[derive(Debug)]
pub struct TempTime {
    pub temp: f64,
    pub time: String,
}

impl TempTime {
    fn new(temp: f64, time: String) -> Self {
        Self { time, temp }
    }
}

fn manipulate_timestr(time: String) -> String {
    let splitvec: Vec<&str> = time.split('T').collect();
    splitvec[1].to_string()
}
