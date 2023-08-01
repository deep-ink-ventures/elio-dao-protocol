# Elio DAO Hookpoints

Decentralized Autonomous Organizations (DAOs) operating in various environments may face vastly different legal and technical demands. These can range from regulatory compliance to specific coding or security protocols. Such customization can be seen in the implementation of distinct structures or governance mechanisms within the DAOs, including vesting structures and boosted voting systems.

Elio DAO is designed with the flexibility to support a plugin system, allowing the seamless integration of various components tailored to individual needs. DAOs working with Elio can deploy custom contracts that satisfy the HookpointsTrait specification, registering them within the set_hookpoint section of the core contract. This permits DAOs to create and utilize features specifically designed for their unique requirements.

Furthermore, Elio DAO's system allows for the interception of the DAO's flow at given points, enabling additional logic, checks, or the implementation of new voting structures. This adaptability ensures that DAOs can operate effectively in alignment with their specific needs and regulatory environment, providing a flexible framework to support the diverse requirements that different DAOs may have.

You can find out an example implementation (`TestHookpointsContract`) in the tests.

## Interface

- `on_before_destroy_dao` - Called before destroying a DAO.
- `on_before_change_owner` - Called before changing the owner of a DAO.
- `on_vote` - Called when a vote for a specific user is casted. Should / can return an adjusted voting amount.
- `on_before_proposal_creation` - Called before proposal creation.
- `on_before_set_metadata` - Called before setting metadata
- `on_set_configuration` - Called when a configuration has been set. Should / can return an adjusted proposal_duration.
- `on_before_fault_proposal` -Called before declaring proposal faulty.
- `on_before_finalize_proposal` Called before finalizing a proposal.
- `on_before_mark_implemented` - Called before marking the proposal implemented.
- `on_incr_allowance` -  Called when assets contract increases allowance.
- `on_decr_allowance` - Called when assets contract decrease allowance.
- `on_xfer` - Called when assets contract is being transferred.
- `on_xfer_from` - Called when assets contract is being transferred for an address.