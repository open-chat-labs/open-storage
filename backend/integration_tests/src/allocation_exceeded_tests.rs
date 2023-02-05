use crate::rng::random_principal;
use crate::setup::{return_env, setup_env, TestEnv};
use crate::{client, tick_many};
use index_canister::add_or_update_users::UserConfig;
use std::time::Duration;

#[test]
fn old_files_deleted_when_allocation_exceeded() {
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
            byte_limit: 1000,
        }],
    );

    let file1 = vec![1u8; 500];
    let file2 = vec![2u8; 500];
    let file3 = vec![3u8; 500];
    let file4 = vec![4u8; 600];

    let allocated_bucket_response1 = client::index::happy_path::allocated_bucket(&env, user_id, index_canister_id, &file1);
    let bucket1 = allocated_bucket_response1.canister_id;
    let file_id1 = allocated_bucket_response1.file_id;
    client::bucket::happy_path::upload_file(&mut env, user_id, bucket1, file_id1, file1, None);

    env.advance_time(Duration::from_millis(1));

    let allocated_bucket_response2 = client::index::happy_path::allocated_bucket(&env, user_id, index_canister_id, &file2);
    let bucket2 = allocated_bucket_response2.canister_id;
    let file_id2 = allocated_bucket_response2.file_id;
    client::bucket::happy_path::upload_file(&mut env, user_id, bucket2, file_id2, file2, None);

    tick_many(&mut env, 10);

    assert!(client::bucket::happy_path::file_exists(&env, user_id, bucket1, file_id1));
    assert!(client::bucket::happy_path::file_exists(&env, user_id, bucket2, file_id2));

    env.advance_time(Duration::from_millis(1));

    let allocated_bucket_response3 = client::index::happy_path::allocated_bucket(&env, user_id, index_canister_id, &file3);
    let bucket3 = allocated_bucket_response3.canister_id;
    let file_id3 = allocated_bucket_response3.file_id;
    client::bucket::happy_path::upload_file(&mut env, user_id, bucket3, file_id3, file3, None);

    tick_many(&mut env, 10);

    assert!(!client::bucket::happy_path::file_exists(&env, user_id, bucket1, file_id1));
    assert!(client::bucket::happy_path::file_exists(&env, user_id, bucket2, file_id2));
    assert!(client::bucket::happy_path::file_exists(&env, user_id, bucket3, file_id3));

    env.advance_time(Duration::from_millis(1));

    let allocated_bucket_response4 = client::index::happy_path::allocated_bucket(&env, user_id, index_canister_id, &file4);
    let bucket4 = allocated_bucket_response4.canister_id;
    let file_id4 = allocated_bucket_response4.file_id;
    client::bucket::happy_path::upload_file(&mut env, user_id, bucket4, file_id4, file4, None);

    tick_many(&mut env, 10);

    assert!(!client::bucket::happy_path::file_exists(&env, user_id, bucket1, file_id1));
    assert!(!client::bucket::happy_path::file_exists(&env, user_id, bucket2, file_id2));
    assert!(!client::bucket::happy_path::file_exists(&env, user_id, bucket3, file_id3));
    assert!(client::bucket::happy_path::file_exists(&env, user_id, bucket4, file_id4));

    return_env(TestEnv {
        env,
        index_canister_id,
        controller,
    });
}
