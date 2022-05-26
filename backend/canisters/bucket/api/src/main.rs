use candid_gen::generate_candid_method;

fn main() {
    generate_candid_method!(bucket, delete_file, update);
    generate_candid_method!(bucket, delete_files, update);
    generate_candid_method!(bucket, upload_chunk_v2, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}
