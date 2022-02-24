pub mod flow {
    use reqwest::header::HeaderMap;
    use reqwest;
    use std::collections::HashMap;

    pub async fn login (email: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        let mut body    = HashMap::new();

        headers.insert("Host", "bfa-login.basic-fit.com".parse().unwrap());
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert("accept", "application/json".parse().unwrap());
        headers.insert("user-agent", "Basic Fit App/1.3.1.0 (iOS)".parse().unwrap());
        headers.insert("accept-language", "en-GB,en;q=0.9".parse().unwrap());

        body.insert("email", email);
        body.insert("password", password);

        let response = reqwest::Client::new().post("https://bfa-login.basic-fit.com/login")
            .headers(headers)
            .send()
            .await?;
        
        println!("{:#?}", response);
        Ok(())
    }
}