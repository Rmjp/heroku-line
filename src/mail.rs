use reqwest::Error;
use std::env;

pub async fn send_mail_verify(user_mail: &String, msg: &String) -> Result<(), Error> {
    let api_key = env::var("mailgun_apikey").expect("mailgun_apikey not found");
    let domain = env::var("mailgun_domain").expect("mailgun_domain not found");
    let send_from = env::var("mailgun_from").expect("mailgun_from not found");
    let url = format!("https://api.mailgun.net/v3/{}/messages", domain);
    let client = reqwest::Client::new();
    let data = [("from", format!("Excited User <{}>", send_from)), ("to", String::from(user_mail)), ("subject", format!("Verify your email")), ("text", format!("This is your number => {}.", msg))];
    let res = client.post(&url)
    .basic_auth("api", Some(api_key))
    .form(&data)
    .send().await?;

    println!("respone: {}", res.text().await?);

    Ok(())
}