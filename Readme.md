Haven
====

**Working in progress**

a SGX app barebone.

- Clone the repo
- `git submodule init`
- `git submodule update`
- `make`
- make sure put `spid.txt` and `key.txt` into `bin/`, the SPID must be linkable
- `cd bin`
- `./app`

POST `/register`

```json
{
  "input": {
    "account": "jasl"
  },
  "nonce": {
    "fuck": "rust"
  }
}
```

POST `/status`

```json
{
  "input": {
    "account": "test"
  },
  "nonce": {
    "fuck": "rust"
  }
}
```

POST `/transfer`

```json
{
  "input": {
    "account": "jasl",
    "to_account": "test",
    "quantity": 100
  },
  "nonce": {
    "fuck": "rust"
  }
}
```
