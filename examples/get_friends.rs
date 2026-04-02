use luduvo_rs::users::friends::FriendsWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = FriendsWrapper::new(None);

    match wrapper.get_friends("1").await {
        Ok(friends) => {
            println!("{:#?}", friends);
        }

        Err(e) => {
            eprintln!("error caught while attempting to get friends: '{}'", e);
        }
    }
}
