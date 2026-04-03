use luduvo_rs::users::friends::FriendsWrapper;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let id = if args.len() < 2 {
        println!("no id supplied, getting profile data for id `1`");

        "1"
    } else {
        &args[1]
    };

    let mut wrapper = FriendsWrapper::new(None);

    match wrapper.get_friends(id).await {
        Ok(friends) => {
            println!("friends for id `{id}`: {:#?}", friends);
        }

        Err(e) => {
            eprintln!(
                "error caught while attempting to get friends for id `{id}`: '{:#?}'",
                e
            );
        }
    }
}
