use luduvo_rs::places::PlacesWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = PlacesWrapper::new(None);

    match wrapper.get_places("test".to_string(), None).await {
        Ok(friends) => {
            println!("places data: {:#?}", friends);
        }

        Err(e) => {
            eprintln!(
                "error caught while attempting to get places`: '{:#?}'",
                e
            );
        }
    }
}
