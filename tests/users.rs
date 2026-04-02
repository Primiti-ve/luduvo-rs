#[allow(unused_comparisons)]
#[cfg(test)]
mod profile {
    use luduvo_rs::users::profile::{ProfileError, ProfileWrapper};

    /// tests that a valid profile can be fetched successfully.
    #[tokio::test]
    async fn get_profile_success() {
        let mut wrapper = ProfileWrapper::new(None);

        // id `1` is a known valid user.
        // this is the `Luduvo` account.
        let id = "1";
        let result = wrapper.get_profile(id).await;

        assert!(result.is_ok(), "expected Ok, got Err: {:?}", result);

        let profile = result.unwrap();

        assert_eq!(profile.user_id.to_string(), id);
        assert!(!profile.username.is_empty());
    }

    /// tests that an invalid profile returns a ProfileNotFound error.
    #[tokio::test]
    async fn get_profile_not_found() {
        let mut wrapper = ProfileWrapper::new(None);

        // id `-1` is a known invalid user.
        // this test assumes that this id does not exist, and will never exist.
        // note(prim): why would -1 exist in the first place???
        let id = "-1";

        match wrapper.get_profile(id).await {
            Err(
                ProfileError::ProfileNotFound(returned_id) | ProfileError::InvalidId(returned_id),
            ) => {
                assert_eq!(returned_id, id);
            }

            err => panic!("expected ProfileNotFound error, got {:?}", err),
        }
    }

    /// tests that the profile api returns a consistent structure.
    #[tokio::test]
    async fn profile_fields_are_valid() {
        let mut wrapper = ProfileWrapper::new(None);
        let id = "1";

        let profile = wrapper.get_profile(id).await.expect("profile should exist");

        // sanity checks
        assert!(!profile.username.is_empty());
        assert!(!profile.display_name.is_empty());

        // avatar checks
        assert!(profile.avatar.head_color.starts_with('#'));
        assert!(profile.avatar.torso_color.starts_with('#'));

        // numeric checks
        assert!(profile.friend_count >= 0);
        assert!(profile.place_count >= 0);
        assert!(profile.item_count >= 0);
    }
}
