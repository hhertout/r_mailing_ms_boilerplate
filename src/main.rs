use mailer;

fn main() {
    dotenvy::dotenv().ok();
    mailer::test_crate();
}
