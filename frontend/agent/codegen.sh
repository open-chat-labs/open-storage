didc bind ../../backend/canisters/bucket/api/can.did -t ts > ./src/services/bucket/candid/types.d.ts
didc bind ../../backend/canisters/bucket/api/can.did -t js > ./src/services/bucket/candid/idl.js

didc bind ../../backend/canisters/index/api/can.did -t ts > ./src/services/index/candid/types.d.ts
didc bind ../../backend/canisters/index/api/can.did -t js > ./src/services/index/candid/idl.js