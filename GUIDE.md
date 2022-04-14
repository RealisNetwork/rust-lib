# GUIDE

All questions note [here](https://docs.google.com/spreadsheets/d/1qRWu9mIqaxKNt8I5E0BFoicpNZHotJhyyrwrFugT_c0/edit?usp=sharing)
(create your own page).

## Rust basics

- Must have to know [rust-book](https://doc.rust-lang.org/book/).
- Alternative to rust-book [rust-by-examples](https://doc.rust-lang.org/stable/rust-by-example/).
- [Rust exercises](https://github.com/rust-lang/rustlings/) (look up for exercises directory).
- Usefully videos, recommended by mentor Alexander - [youtube channel](https://www.youtube.com/c/JonGjengset/videos).


## Blockchain basics

- [What Is a Blockchain?](https://www.investopedia.com/terms/b/blockchain.asp)

## Substrate

- [Installation guide](https://docs.substrate.io/v3/getting-started/installation/)
- [Tutorials](https://docs.substrate.io/tutorials/v3/)
  1. [Create your first substrate chain](https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/)
  2. [Add the Niks pallet to your Runtime](https://docs.substrate.io/tutorials/v3/add-a-pallet/)
  3. [Start a private nerwork](https://docs.substrate.io/tutorials/v3/private-network/)
  4. [Substrate Kitties workshop](https://docs.substrate.io/tutorials/v3/kitties/pt1/)
- [Key concepts](https://docs.substrate.io/v3/concepts/runtime/)
  - Runtime
  - Extrinsics
  - Account
  - Transaction pool
  - Session keys
  - Weight
  - Execution
  - Off-chain *(Optional)*
- Storages
  - [Theory](https://docs.substrate.io/v3/advanced/storage/)
  - [Api](https://docs.substrate.io/v3/runtime/storage/)

## Git

[Tutorial](https://www.w3schools.com/git/git_remote_getstarted.asp?remote=github)

## Other necessary software

Recommend to use Ubuntu

- [Docker](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04-ru)
- Docker-compose
  ```shell
    sudo apt update
    sudo apt install docker-compose
  ```
- Clion. Can be installed with Ubuntu software.
- [Postgres](https://www.digitalocean.com/community/tutorials/how-to-install-postgresql-on-ubuntu-20-04-quickstart-ru)
  Also you should change postgres user password to postgres, for this use following comands:
  ```shell
    sudo -u postgres psql
  ```
  ```sql
    ALTER USER postgres WITH PASSWORD 'postgres';
  ```
- DataGrip. Can be installed with Ubuntu software.

# Architecture

## Nats

Basicly we use microservice architecture. Serviceses communicate with one an other throw the nats-streaming.
Nats is the service which work like a post man - it takes the message and the address(in nats it calls `topic`) than it deliver the message by this address.
Nats don't stop delivering message until while recipient, the recipient must say to nats that he got the message(in nats it calls `ok-ing` or `acknowledge`).

## Request-response protocol

All services communicate throw request-response. Request and response presented as JSON.
Each request has its own unique id represented as string, in rust services we use uuid but it can be any string.

Basic request structure:

```javascript
{
  "id": "some_unique_id",
  "topicRes": "topic_to_response", // can be missed if no need to response or response by hard coded topic
  "params": {
    // ... // here will be listed unique for this request params
  },
  "authInfo": {
    "userId": "some_user_id", // if request elate to specific user
    // some field omitted
  }
}
```


Basic response structure:

- Error:
  ```javascript
  {
    "result": {
      "request": "...", // request that provoked this response
      "response": {
        "type": "Left",
        "value": {
          "msg": "error 404",
          "error_type": "", // specific error type
          "trace?": "",
          "data?": {"..."},
          "status?": "..."
        }
      }
    }
  }
  ```
- Success:
  ```javascript
  {
    "result": {
      "request": "...", // request that provoked this response
      "response": {
        "type": "Left",
        "value": {
          "..." // response data
        }
      }
    }
  }
  ```


For now there are 4 main services:
1. realis-wallet
  Create for user account on blockcain save private(encoded) and public key in database.
2. realis-adapter
  Collect transactions in a batch and send to the blockchain.
3. orchestrator
  Redirect requests and responses, add blochain account data to request by `userId`.
4. realis-listener
  Track blockchain blocks and notify if something interesting appear.
