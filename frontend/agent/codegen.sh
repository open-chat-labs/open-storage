rm ./src/services/bucket/candid/types.ts
rm ./src/services/bucket/candid/idl.js
didc bind ../../backend/canisters/bucket/api/can.did -t ts >> ./src/services/bucket/candid/types.ts
didc bind ../../backend/canisters/bucket/api/can.did -t js >> ./src/services/bucket/candid/idl.js

rm ./src/services/index/candid/types.ts
rm ./src/services/index/candid/idl.js
didc bind ../../backend/canisters/index/api/can.did -t ts >> ./src/services/index/candid/types.ts
didc bind ../../backend/canisters/index/api/can.did -t js >> ./src/services/index/candid/idl.js