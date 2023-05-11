# Elio DAO

## An extendable DAO Ecosystem

Welcome to the Elio DAO Protocol.

> This protocol is under heavy development. Don't try this at home.

Decentralized Autonomous Organizations (DAOs) represent a paradigm shift in the way organizations are managed, leveraging blockchain technology to enable decentralized decision-making, community governance, and shared ownership. DAOs operate through smart contracts, automating transactions and organizational processes, thus minimizing the need for centralized authorities.

The standalone protocol can be used to manage funds, communities and other decentralized organizations without a single line of code to be written.

The twist for Elio DAO, though, is its extendable hook point functionality, where all protocol contracts allow developers to provide extensions in the form of soroban smart contracts that are called from within the lifecycle of the protocol.

> The current state of the protocol is the core development. We'll add hook point functionality shortly.

This enables a plugin-like system that people already know from web2 apps like wordpress and shopify. Elio DAO is not only a protocol, but an ecosystem that can be extended easily and has composition as a first class citizen of its protocol. Developers (and ourselves) can customize DAOs to legal needs or community requirements without any limits.

## Overview

The protocol consists of three main contracts.

![image](https://github.com/deep-ink-ventures/elio-dao-protocol/assets/120174523/99ccedac-ea58-4f0b-bee2-f274ee70cc59)

[**Elio DAO Core**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/core) is the central entity of Elio DAO to create and manage DAOs. It is as well the place where a DAO can issue it's token that will become one of ...

[**Elio DAO Assets**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/assets), a ERC-20 style token with checkpoint functionality, supporting the [Soroban Token Interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).

> _Assets_ is the bread and butter of our first deliverable in the SCF Community Grant. You can read more about it in it's [README](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/assets/README.md)!

[**Elio DAO Votes**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/votes) is the proposal lifecycle contract with built in voting functionality.

## What else?

We're warming up for the dApp and ecosystem development! 

Be sure to follow [the backend service](https://github.com/deep-ink-ventures/elio-dao-service) of our protocol that parses the soroban event stream and abstracts aways some of the nitty gritty protocol details. 

It provides a clean and fun to use API for our [frontend](https://github.com/deep-ink-ventures/elio-dao-frontend).

If you are more on the visual side of things, [here](https://www.figma.com/file/25eK8qARqvKX9ZMtIHbc3U/Design-Deck?type=design&node-id=126-6939) is [a](https://www.figma.com/file/25eK8qARqvKX9ZMtIHbc3U/Design-Deck?type=design&node-id=16-103) sneak [peak](https://www.figma.com/file/25eK8qARqvKX9ZMtIHbc3U/Design-Deck?type=design&node-id=2-101) into [what](https://www.figma.com/file/25eK8qARqvKX9ZMtIHbc3U/Design-Deck?type=design&node-id=34-697) we are building.

## Get involved!

### Contract Development
The local development is pretty straightforward.

Follow the [setup](https://soroban.stellar.org/docs/getting-started/setup) for soroban and `cd` into a contract of choice.

Run the tests:

```sh
cd contracts/assets
cargo test
```

The contract within this protocol are loosely coupled and they need a wasm blob to [wire](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/assets/src/test.rs#L19-L31). Therefore, after each code adjustments you need to run

```sh
./init.sh
```

in the root to update to the latest iteration.

### Run a local network

Start by running the official soroban docker container:

```sh
docker run --rm -it \
   -p 8000:8000 \
   --name stellar \
   stellar/quickstart:soroban-dev@sha256:a057ec6f06c6702c005693f8265ed1261e901b153a754e97cf18b0962257e872 \
   --standalone \
  --enable-soroban-rpc
  ```

This may take A WHILE. Get a coffee until you see `INFO success: soroban-rpc entered RUNNING state` in the console.

Copy `.env.example` to `.env` and generate a keypair [here](https://laboratory.stellar.org/#account-creator?network=futurenet).

Fund the account via curl:

```sh
curl "http://localhost:8000/friendbot?addr=${PUBLIC_KEY}"
```

Run `./deploy.sh` - this will deploy the latest and greatest from the `wasm` folder. Write down the wasm hash for future interactions.

### Deploy to futurenet

Copy `.env.example` to `.env` and generate a keypair [here](https://laboratory.stellar.org/#account-creator?network=futurenet).

Fund the account either on the same website where you generated the keypair or via curl:

```sh
curl "https://friendbot-futurenet.stellar.org?${PUBLIC_KEY}
```

Run `./deploy.sh` - this will deploy the latest and greatest from the `wasm` folder. Write down the wasm hash for future interactions.







