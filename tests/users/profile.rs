use luduvo_rs::users::profile::{ProfileError, ProfileWrapper};
use tokio::time;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn setup_wrapper(server: &MockServer) -> ProfileWrapper {
    ProfileWrapper::new_with_base_url(Some(60), format!("{}/users", server.uri()))
}

fn mock_profile_body() -> serde_json::Value {
    json!({
        "user_id": 1,
        "username": "Luduvo",
        "display_name": "Luduvo",

        "bio": None::<String>,
        "status": None::<String>,

        "avatar": {
            "head_color": "#FFFFFF",
            "torso_color": "#FFFFFF",
            "left_arm_color": "#FFFFFF",
            "right_arm_color": "#FFFFFF",
            "left_leg_color": "#FFFFFF",
            "right_leg_color": "#FFFFFF"
        },

        "equipped_items": [],
        "badges": [],

        "friend_count": 0,
        "place_count": 0,
        "item_count": 0,

        "last_active": None::<u64>,
        "member_since": None::<u64>,

        "allow_joins": true,
        "is_owner": false
    })
}

/// success case
#[tokio::test]
async fn get_profile_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_profile_body()))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let profile = wrapper.get_profile("1").await.unwrap();

    assert_eq!(profile.user_id, 1);
    assert_eq!(profile.username, "Luduvo");
}

/// not found
#[tokio::test]
async fn get_profile_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999/profile"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_profile("999").await {
        Err(ProfileError::ProfileNotFound(id)) => assert_eq!(id, "999"),
        
        other => panic!("expected ProfileNotFound, got {:?}", other),
    }
}

/// invalid id
#[tokio::test]
async fn get_profile_invalid_id_format() {
    let mut wrapper = ProfileWrapper::new(None);

    match wrapper.get_profile("abc").await {
        Err(ProfileError::InvalidId(id)) => assert_eq!(id, "abc"),
        
        other => panic!("expected InvalidId, got {:?}", other),
    }
}

/// cache test
#[tokio::test]
async fn get_profile_cache_hit() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_profile_body()))
        .expect(1)
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let _ = wrapper.get_profile("1").await.unwrap();
    let _ = wrapper.get_profile("1").await.unwrap();
}

/// structure validation
#[tokio::test]
async fn profile_fields_are_valid() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_profile_body()))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    let profile = wrapper.get_profile("1").await.unwrap();

    assert!(!profile.username.is_empty());
    assert!(!profile.display_name.is_empty());

    assert!(profile.avatar.head_color.starts_with('#'));
    assert_eq!(profile.avatar.head_color.len(), 7);
}

/// server error (500)
#[tokio::test]
async fn get_profile_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_profile("1").await {
        Err(ProfileError::RequestFailed(_)) => {}
        
        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

/// invalid json response
#[tokio::test]
async fn get_profile_invalid_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);

    match wrapper.get_profile("1").await {
        Err(ProfileError::RequestFailed(_)) => {}
        
        other => panic!("expected RequestFailed, got {:?}", other),
    }
}

/// tests optional fields are actually None
#[tokio::test]
async fn profile_optional_fields_none() {
    let server = MockServer::start().await;
    let mut body = mock_profile_body();
    
    body["bio"] = serde_json::Value::Null;
    body["status"] = serde_json::Value::Null;
    body["last_active"] = serde_json::Value::Null;
    body["member_since"] = json!(1000);

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let mut wrapper = setup_wrapper(&server);
    let profile = wrapper.get_profile("1").await.unwrap();

    assert!(profile.bio.is_none());
    assert!(profile.status.is_none());
    assert!(profile.last_active.is_none());
}

/// cache expiration test
#[tokio::test]
async fn get_profile_cache_expiration() {
    let server = MockServer::start().await;

    let body = {
        let mut mock_body = mock_profile_body();
        
        mock_body["member_since"] = json!(1000);
        mock_body
    };

    Mock::given(method("GET"))
        .and(path("/users/1/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(2)
        .mount(&server)
        .await;

    let mut wrapper = ProfileWrapper::new_with_base_url(
        Some(1),
        format!("{}/users", server.uri()),
    );

    let _ = wrapper.get_profile("1").await.unwrap();

    time::sleep(std::time::Duration::from_secs(2)).await;

    let _ = wrapper.get_profile("1").await.unwrap();
}
