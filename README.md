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


