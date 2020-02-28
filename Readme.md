pRuntime
====

**Working in progress**

This is the implementation of pRuntime (Phala Network TEE Runtime) based on Intel SGX. It's the
runtime to execute confidential smart contracts.

The runtime contains the following components:

- Identity manager
- Remote Attestation module
- Offchain bridge (by a Substrate light client)
- Confiidential contract executor & state manager
- Restful RPC service

## Build

- Clone the repo
- `git submodule init`
- `git submodule update`
- `make`
- Make sure put `spid.txt` and `key.txt` into `bin/`, the SPID must be linkable
- `cd bin`
- `./app`

Intel SGX Driver and SDK are needed. Set environment variable `SGX_MODE=SW` while building to run
it in computer without SGX.

The dev mode keys in `spid.txt` and `key.txt` can be obtainied [from Intel](https://software.intel.com/en-us/sgx/attestation-services).

## RPC

Helper script: `script/console.sh`

```bash
./script/console.sh get_info
./script/console.sh init
./script/console.sh dump_states
```
