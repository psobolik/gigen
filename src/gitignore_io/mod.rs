pub mod error;

use hyper::{Client, StatusCode};
use hyper_tls::HttpsConnector;
use std::str;
use crate::gitignore_io::error::Error;

pub async fn get_template_names() -> Result<Vec<String>, Error> {
    let mut vec = Vec::new();
    let response = invoke_api("list").await?;
    for lines in response.split('\n') {
        for template in lines.split(',') {
            vec.push(template.to_string());
        }
    }
    Ok(vec)
}

pub async fn get_template(template_names: Vec<&str>) -> Result<String, Error> {
    let response = invoke_api(&template_names.join(",")).await?;
    Ok(response)
}

async fn invoke_api(method: &str) -> Result<String, Error> {
    let url = format!("https://www.toptal.com/developers/gitignore/api/{}", method).parse().unwrap();

    let connector = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(connector);

    let response = client.get(url).await?;
    if response.status() == StatusCode::OK {
        let body = response.into_body();
        let bytes = hyper::body::to_bytes(body).await?;

        return Ok(String::from_utf8(bytes.to_vec()).unwrap());
    }
    Err(Error::new(&response.status().to_string()))
}
