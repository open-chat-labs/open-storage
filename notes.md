## Open-Storage

Notes
-----
- each app runs its own OpenStorage for now
- for content addressing to work we need to use the allocated data bucket for the particular blob hash
- 1 data_bucket_index canister
- multiple data_bucket canisters
- data_buckets can be in separate subnets
- data *upload* allowance per user - not related to groups/chats
- when a user is registered with OC it is also registered (non-blocking) with the data_bucket_index which includes the data allowance
- if the user goes premium a call is made to data_bucket_index to increase their data allowance
- the BlobId is the hash of the blob data

Uploading a blob from FE
------------------------
1. FE derives BlobId by calculating hash of blob data using OpenStorage js lib
2. FE queries data_bucket_index::allocated_bucket with the BlobId to get the canister_id of the allocated data_bucket
3. FE calls data_bucket::push_chunk with multiple chunks in parallel - includes a vec of principals that can access the blob (accessor_ids)
4. FE calls group::send_message with the BlobReference

Rendering a message from FE
---------------------------
- chat::message -> {
    blob_id, 
    data_bucket_canister_id, 
    token={your principal encrypted by chat::canister - where canister principal is the public key}
}
Construct url like: 
http://{data_bucket_canister_id}.ic0.app/{accessor_id}/{blob_id}?token=<TOKEN>

Deleting a message from FE
--------------------------
1. Call chat::delete_message ->
{ 
    blob_id, 
    data_bucket_canister_id, 
    token={your principal encrypted by chat::canister} 
}
2. Call data_bucket::delete_blob

Deleting a chat from FE
-----------------------
1. Call chat::delete ->
{ 
    token={your principal encrypted by chat::canister} 
}
2. Call data_bucket_index::delete_chat({chat_id, token})



data_bucket::push_chunk
-----------------------
1. Is the caller authorized to push_chunk?
2. Does the blob already exist? Goto 6
3. Does the chunk already exist?
4. Will the blob exceed the caller's data allowance
5. If last chunk - does the hash match the hash provided by the client? If not then throw away all the chunks.
6. When a new blob has been fully uploaded (or already exists)
 - Add a reference to the blob with the user's principal and the "owner(chat) principal" this blob is visible to
 - a non-blocking call is made to the data_bucket_index to decrement the user's "bytes left"

data_bucket::http_request
-------------------------
1. Decrypt the token and confirm the principal matches the caller
2. Lookup the blob by id
3. Confirm the "chat_id" matches a reference held against the blob

data_bucket::upload_chunk
data_bucket::add_blob_reference
data_bucket::remove_blob_reference
data_bucket::http_request
data_bucket::c2c_register_user
data_bucket::c2c_users_updated
data_bucket::c2c_delete_chat
    delete_accessor?


data_bucket_index
-----------------
pub struct Data {
    pub users: HashMap<UserId, UserRecord>,
    pub blobs: HashMap<Hash, BlobRecord>,
    pub active_buckets: Vec<BucketRecord>,
    pub full_buckets: Vec<BucketRecord>,
}

pub struct UserRecord {
    pub byte_limit: u64,
    pub bytes_used: u64,
}

pub struct BlobRecord {
    pub bucket: CanisterId,
    pub size: u64,
}

pub struct BucketRecord {
    pub canister_id: CanisterId,
    pub bytes_used: u64,
}

register_user
update_user
delete_user
allocated_bucket(blob_id)
c2c_add_blob_reference
c2c_remove_blob_reference
heartbeat (topup buckets, upgrade buckets, push registered users, and push updated "bytes left" for each user in batches to each data_bucket)