# Elio DAO

## An extendable DAO Ecosystem

Welcome to the Elio DAO Protocol.

Decentralized Autonomous Organizations (DAOs) represent a paradigm shift in the way organizations are managed, leveraging blockchain technology to enable decentralized decision-making, community governance, and shared ownership. DAOs operate through smart contracts, automating transactions and organizational processes, thus minimizing the need for centralized authorities.

The standalone protocol can be used to manage funds, communities and other decentralized organizations without a single line of code to be written.

The twist for Elio DAO, though, is its extendable hook point functionality, where all protocol contracts allow developers to provide extensions in the form of soroban smart contracts that are called from within the lifecycle of the protocol.

This enables a plugin-like system that people already know from web2 apps like wordpress and shopify. Elio DAO is not only a protocol, but an ecosystem that can be extended easily and has composition as a first class citizen of its protocol. Developers (and ourselves) can customize DAOs to legal needs or community requirements without any limits.

## Overview

The protocol consists of three main contracts and an optional Hookpoint contract that each DAO can configure.

![image](https://github.com/deep-ink-ventures/elio-dao-protocol/assets/120174523/a4b51d4a-1da0-4c70-b3cf-b9d525b8f771)


[**Elio DAO Core**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/core) is the central entity of Elio DAO to create and manage DAOs. It is as well the place where a DAO can issue it's token that will become one of ...

[**Elio DAO Assets**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/assets), an ERC-20 style token with checkpoint functionality, supporting the [Soroban Token Interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).

[**Elio DAO Votes**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/votes) is the proposal lifecycle contract with built in voting functionality.

[**Elio DAO Hookpoints**](https://github.com/deep-ink-ventures/elio-dao-protocol/tree/main/contracts/hookpoints) is the template for a contract that each DAO can optionally deploy to intercept and alter the behaviour of the protocol for it's DAO.

## What else?

This is not only a protocol, it's a platform, dApp and service.

Be sure to follow [the backend service](https://github.com/deep-ink-ventures/elio-dao-service) of our protocol that parses the soroban event stream and abstracts aways some of the nitty gritty protocol details. 

It provides a clean and fun to use API for our [frontend](https://github.com/deep-ink-ventures/elio-dao-frontend).

## Testnet

A full testnet version is deployed available:

- The dApp lives at https://elio-dao.org/
- The service lives at https://service.elio-dao.org/
- The current live contracts can be found here: https://service.elio-dao.org/config/
- The API is documented here: https://service.elio-dao.org/redoc/

## Get involved!

### Contract Development
The local development is pretty straightforward.

Follow the [setup](https://soroban.stellar.org/docs/getting-started/setup) for soroban and `cd` into a contract of choice.

Run the tests:

```sh
cd contracts/assets
cargo test
```

The contracts within this protocol are loosely coupled and they need a wasm blob to [wire](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/assets/src/test.rs#L19-L31). Therefore, after each code adjustment you need to run

```sh
./init.sh
```

in the root to update to the latest iteration.

### Deploy to futurenet

Copy `.env.example` to `.env` and generate a keypair [here](https://laboratory.stellar.org/#account-creator?network=futurenet).

Fund the account either on the same website where you generated the keypair or via curl:

```sh
curl "https://friendbot-futurenet.stellar.org?${PUBLIC_KEY}
```

Run `./deploy.sh` - this will deploy the latest and greatest from the `wasm` folder.

> By default the deploy script notifies our backend service of the new deployment, which in turn requires a secret key.
> If you just want to test the contract, just comment out [the respective lives](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/deploy.sh#L85-L92).
> You can as well run your own service, which is explained [here](https://github.com/deep-ink-ventures/elio-dao-service).

### Bootstrap the protocol

Once you have setup the protocol, the core contract is deployed and the votes and asset contracts are installed - they'll
be deployed by the core contract.

Source the env file:

```sh
source .env
```

Now you can create your first dao.

> Note how we are converting string to bytes, as the protocol requests with `echo 'some string' | xxd -p`
>
> String to bytes: `echo 'Deep Ink Ventures' | xxd -p`
> → Outputs `4465657020496e6b2056656e74757265730a`
>
> Bytes to string: `echo '4465657020496e6b2056656e74757265730a' | xxd -r -p`
> → Outputs `Deep Ink Ventures`

Run:

```sh
soroban contract invoke \
   --id ${CORE_ADDRESS} \
   --source ${SECRET_KEY} \
   --rpc-url ${RPC_URL} \
   --network-passphrase "${NETWORK_PASSPHRASE}" \
   -- \
   create_dao \
   --dao_id `echo 'DIV' | xxd -p` \
   --dao_name `echo 'Deep Ink Ventures' | xxd -p` \
   --dao_owner ${PUBLIC_KEY}
```

This should give you your dao back:

```json
{
   "id":"4449560a",
   "name":"4465657020496e6b2056656e74757265730a",
   "owner":"GA455NS7JA6KVEAN6PFCJF3IELZ2A2ABMO7WVRY75ZYMOJFWL2VTK77P"
}
```

Congrats, DAO Owner!
