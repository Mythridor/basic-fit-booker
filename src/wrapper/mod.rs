mod wrapper {
    pub mod flow {

        #[tokio::main]
         pub async fn login (email: String, password: String) -> Result<(), Box<dyn std::error::Error>> {

        let mut body = object!{
            "email": email
            "password": password
        };

        let mut headers = object!{
            "Host": "bfa-login.basic-fit.com",
            "content-type": "application/json",
            "accept": "application/json",
            "user-agent": "Basic Fit App/1.3.1.0 (iOS)",
            "accept-language": "en-GB,en;q=0.9",
        };

        let mut client = reqwest::Client::new();

        let resp = client::post("https://bfa-login.basic-fit.com/login")
            .body(body)
            .headers(headers)
            .send()
            .await?
            .json::<std::collections::HashMap<String, String>>()
            .await?; 

        println!("{:#?}", resp);
        Ok(())
    }
}
}