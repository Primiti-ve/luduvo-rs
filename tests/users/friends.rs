use luduvo_rs::users::friends::{FriendsError, FriendsWrapper};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_wrapper(server: &MockServer) -> FriendsWrapper {
    FriendsWrapper::new_with_base_url(Some(60), format!("{}/users", server.uri()))
}

/// tests that valid friends data can be fetched
#[tokio::test]
async fn get_friends_success() {
    let server = MockServer::start().await;

    let body = json!({
        "friends": [],
        "total": 0,
        "limit": 50,
        "offset": 0
    });

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let friends = wrapper.get_friends("1").await.unwrap();

    assert_eq!(friends.total, 0);
    assert_eq!(friends.friends.len(), 0);
}

/// tests invalid id format
#[tokio::test]
async fn get_friends_invalid_id() {
    let mut wrapper = FriendsWrapper::new(None);

    match wrapper.get_friends("abc").await {
        Err(FriendsError::InvalidId(id)) => assert_eq!(id, "abc"),
        other => panic!("expected InvalidId, got {:?}", other),
    }
}

/// tests not found case
#[tokio::test]
async fn get_friends_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999/friends"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_friends("999").await {
        Err(FriendsError::ResultNotFound(id)) => assert_eq!(id, "999"),
        other => panic!("expected ResultNotFound, got {:?}", other),
    }
}

/// tests cache behavior (ensures only one http call happens)
#[tokio::test]
async fn get_friends_cache_hit() {
    let server = MockServer::start().await;

    let body = json!({
        "friends": [],
        "total": 0,
        "limit": 50,
        "offset": 0
    });

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let first = wrapper.get_friends("1").await.unwrap();
    let second = wrapper.get_friends("1").await.unwrap();

    assert_eq!(first.total, second.total);
}

/// tests pagination sanity
#[tokio::test]
async fn friends_pagination_sanity() {
    let server = MockServer::start().await;

    let body = json!({
        "friends": [],
        "total": 0,
        "limit": 50,
        "offset": 0
    });

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let friends = wrapper.get_friends("1").await.unwrap();

    assert!(friends.limit > 0);
    assert!(friends.offset <= friends.total);
}

/// server error (500)
#[tokio::test]
async fn get_friends_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_friends("1").await {
        Err(FriendsError::RequestFailed(_)) => {}
        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

/// invalid json
#[tokio::test]
async fn get_friends_invalid_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid"))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_friends("1").await {
        Err(FriendsError::RequestFailed(_)) => {}
        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

/// cache expiration
#[tokio::test]
async fn get_friends_cache_expiration() {
    let server = MockServer::start().await;

    let body = json!({
        "friends": ["2"],
        "total": 1,
        "limit": 50,
        "offset": 0
    });

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(2)
        .mount(&server)
        .await;

    let mut wrapper = FriendsWrapper::new_with_base_url(Some(1), format!("{}/users", server.uri()));
    let _ = wrapper.get_friends("1").await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let _ = wrapper.get_friends("1").await.unwrap();
}

/// large dataset consistency
#[tokio::test]
async fn friends_large_dataset_consistency() {
    let server = MockServer::start().await;

    let friends_list: Vec<String> = (0..100).map(|i| i.to_string()).collect();
    let body = json!({
        "friends": friends_list,
        "total": 100,
        "limit": 100,
        "offset": 0
    });

    Mock::given(method("GET"))
        .and(path("/users/1/friends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let friends = wrapper.get_friends("1").await.unwrap();

    assert_eq!(friends.friends.len(), 100);
    assert_eq!(friends.total, 100);
}
