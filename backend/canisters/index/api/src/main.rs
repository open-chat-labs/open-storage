use candid_gen::generate_candid_method;

fn main() {
    generate_candid_method!(index, allocated_bucket_v2, query);
    generate_candid_method!(index, can_forward, query);
    generate_candid_method!(index, user, query);

    generate_candid_method!(index, add_or_update_users, update);
    generate_candid_method!(index, remove_accessor, update);
    generate_candid_method!(index, remove_user, update);
    generate_candid_method!(index, update_user_id, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}
