use crate::client;
use crate::rng::random_principal;
use crate::setup::{return_env, setup_env, TestEnv};
use index_canister::add_or_update_users::UserConfig;
use utils::hasher::hash_bytes;

#[test]
fn upload_file() {
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
    let file_hash = hash_bytes(&file);
    let file_size = file.len() as u64;

    let bucket = client::index::happy_path::allocated_bucket(&env, user_id, index_canister_id, &file).canister_id;

    let file_id = client::bucket::happy_path::upload_file(&mut env, user_id, bucket, file, None);

    let file_info_response = client::bucket::happy_path::file_info(&env, user_id, bucket, file_id);

    assert!(file_info_response.is_owner);
    assert_eq!(file_info_response.file_hash, file_hash);
    assert_eq!(file_info_response.file_size, file_size);

    let user_response = client::index::happy_path::user(&env, user_id, index_canister_id);

    assert_eq!(user_response.bytes_used, file_size);

    return_env(TestEnv {
        env,
        index_canister_id,
        controller,
    });
}
