use crate::rng::random_principal;
use crate::setup::{return_env, setup_env, TestEnv};
use crate::{client, tick_many};
use index_canister::add_or_update_users::UserConfig;
use std::time::{Duration, SystemTime};
use types::TimestampMillis;

#[test]
fn file_is_removed_after_expiry_date() {
    let TestEnv {
        mut env,
        index_canister_id,
        controller,
    } = setup_env();

    let user_id = random_principal();
    client::index::happy_path::add_or_update_users(
        &mut env,
        controller,
        index_canister_id,
        vec![UserConfig {
            user_id,
            byte_limit: 10000,
        }],
    );

    let file = vec![1u8; 1000];

    let bucket = client::index::happy_path::allocated_bucket(&env, user_id, index_canister_id, &file).canister_id;

    let now: TimestampMillis = env.time().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;

    let file_id = client::bucket::happy_path::upload_file(&mut env, user_id, bucket, file, Some(now + 1000));

    env.advance_time(Duration::from_millis(999));
    env.tick();

    let file_info_response1 = client::bucket::file_info(&env, user_id, bucket, &bucket_canister::file_info::Args { file_id });
    assert!(matches!(
        file_info_response1,
        bucket_canister::file_info::Response::Success(_)
    ));

    env.advance_time(Duration::from_millis(1));
    tick_many(&mut env, 5);

    let file_info_response2 = client::bucket::file_info(&env, user_id, bucket, &bucket_canister::file_info::Args { file_id });
    assert!(matches!(file_info_response2, bucket_canister::file_info::Response::NotFound));

    let user_response = client::index::happy_path::user(&env, user_id, index_canister_id);

    assert_eq!(user_response.bytes_used, 0);

    return_env(TestEnv {
        env,
        index_canister_id,
        controller,
    });
}
