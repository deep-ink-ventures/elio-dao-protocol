# DAO Votes Contract

This contract manages the proposal lifecycle of a DAO and manages the voting functionality.

## Interface

- `init` - initialize the contract, that is done upon protocol deployment
- `get_core_id` - address of the used core contract
- `create_proposal` - create a new proposal for the dao
- `set_metadata` - set the metadata url (normally links to a web or ipfs url, see our service for an example) and a hash of the metadata 
- `get_metadata` - retrieve the metadata
- `get_active_proposals` - retrieve the active proposals
- `get_archived_proposal` - retrieve archived proposals (note that those may expire)
- `set_configuration` - set the dao specific configuration for the proposal management such as duration and threshold 
- `get_configuration` - retrieve the configuration
- `has_configuration` - find out if a configuration is set for a dao
- `remove_configuration` - remove the config for a dao
- `vote` - vote on a proposal
- `fault_proposal` - set the proposal as faulty if it's spam or malicious 
- `finalize_proposal` - update the state to final once a proposal is passed it's voting period 
- `mark_implemented` - marks the proposal as completed once the requested changes are implemented