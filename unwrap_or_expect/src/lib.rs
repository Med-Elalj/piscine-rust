pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match (server, security_level) {
        (Ok(url), Security::Unknown)
        | (Ok(url), Security::Message)
        | (Ok(url), Security::Warning)
        | (Ok(url), Security::NotFound) => {
            url.to_string()
        }
        (Ok(url), Security::UnexpectedUrl) => {
            panic!("{}", url);
        }
        (Err(msg), Security::Unknown) => {
            Err::<&str, &str>(msg).unwrap().to_string()
        }
        (Err(_msg), Security::Message) => {
            panic!("ERROR: program stops");
        }
        (Err(_msg), Security::Warning) => {
            "WARNING: check the server".to_string()
        }
        (Err(msg), Security::NotFound) => {
            format!("Not found: {}", msg)
        }
        (Err(msg), Security::UnexpectedUrl) => {
            msg.to_string()
        }
    }
}
