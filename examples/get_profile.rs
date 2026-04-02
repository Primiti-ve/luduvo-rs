use luduvo_rs::users::profile::ProfileWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None);

    match wrapper.get_profile("1").await {
        Ok(profile) => {
            println!("{:#?}", profile);
        }

        Err(e) => {
            eprintln!("error caught while attempting to get profile: '{}'", e);
        }
    }
}
