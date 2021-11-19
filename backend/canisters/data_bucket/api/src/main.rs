use candid_gen::generate_candid_method;

fn main() {
    generate_candid_method!(data_bucket, put_chunk, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}
