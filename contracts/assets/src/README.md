# DAO Assets Contract
...

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