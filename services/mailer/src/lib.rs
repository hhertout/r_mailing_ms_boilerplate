pub struct Mailer {
   from: String 
}

impl Mailer {
    pub fn new() -> Mailer {
        let from = std::env::var("SMTP_FROM").unwrap_or_else(|_| panic!("Failed to load env variable"));
        Mailer {from}
    }
}

pub fn test_crate() {
    let test = std::env::var("SMTP_FROM");
    println!("Hello, world!, {:?}", test);

}
