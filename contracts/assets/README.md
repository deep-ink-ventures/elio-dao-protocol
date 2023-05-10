# DAO Assets Contract
The DAO Assets Contract is a token to be used within the Elio DAO Protocol. It is a reference token for DAO capabilities.

> You can absolutely roll your own token by implementing the same trait; specifically our protocol is interested in the `get_balance_at` function that we'll detail out here.

## Checkpoints
The basic idea of this token is to provide checkpoint functionality. The idea here is to freeze and reference the voting power before a proposal is created. We do so by writing a checkpoint on all [balance changes](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/assets/src/types.rs#L178).

![image](https://github.com/deep-ink-ventures/elio-dao-protocol/assets/120174523/c72cbc3e-a992-4adc-8e8f-e36e49a085a7)

The checkpoint system maintains a record of token balances at specific points in time, referred to as [*checkpoints*](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/assets/src/types.rs#L31-L36). Each checkpoint represents an accounts's token balance at a particular ledger sequence number (a ledger in the Stellar blockchain). The smart contract [keeps a mapping](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/assets/src/types.rs#L28) of addresses to an array of checkpoints, which store the balance and the associated ledger sequence number.

To ensure scalability and efficiency, a history horizon is [implemented](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/assets/src/types.rs#L89) based on the [maximum proposal duration and the maximum number of proposals allowed](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/votes/src/types.rs#L25-L43). This approach keeps the checkpoint growth bounded, preventing excessive storage requirements and allowing for more efficient lookup and processing of account balances during the voting process.

This process allows accounts to only signal favor/not-in-favor for a proposal but they do not have to lock up tokens during the voting process - which is known to be discouraging from voting. By taking advantage of the checkpoints, we will prevent double counting, as all votes will reference the same checkpoint, preventing that one user can vote and transfer to a different account and vote again.

Since Elio DAO will provide a flexible extension system, users can use this transparent token that allows for fully on-chain governance - but they don’t have to. This is a basic building block of Elio DAOs core functionality but it can be replaced with [custom solutions](https://github.com/deep-ink-ventures/elio-dao-protocol/blob/main/contracts/core/src/lib.rs#L57-L59) due to our modular approach.

## Upgrading and protocol switches
While we're obviously thrilled if you use **Elio DAO** for your DAO management purposes, you can at all times swap the underlying governance system via the `set_governance_id` function. This as well allows upgrading to newer deployed versions of our protocol. The `owner` is independent of the `Elio DAO Core` manager (though initially and practially the same most of the time), so that assets maintain integrity even when Elio DAO is for whatever reason no longer available.

### Interface
- `init`: Constructor of the assets
- `get_balance_at`: Get the last recorded historical balance at or before the given ledger sequence number
- `get_checkpoint_count`: Discovery Function: Get the number of checkpoints stored for a given id
- `get_checkpoint_at`: Discovery Function: Get a checkpoint at an index stored for a given id
- `set_owner`: Change the owner of this token
- `owner`: Returns the current owner
- `set_governance_id`: Change the governance id of this token to either a different implementation or to upgrade to a newer version of elio DAO.
- `governance_id`: Returns the current governance id.

The remainder of the interface follows the [Soroban Token Interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).

Notably: `decimals`is always 18, `spendable` always equals balance and `authorized` is always true.
