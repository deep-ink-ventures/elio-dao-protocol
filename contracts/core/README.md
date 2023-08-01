# DAO Core Contract
Create a DAO and run.

## Overview
This module contains functionality to create, manage and destroy a DAO alongside with token issuance.
It acts as a central actor of the protocol and provides configuration features and smart contract hook points to fine-tune and customize a DAO with great freedom.

### Interface
- `init`: Initialize the contract, this is done on protocol deployment
- `create_dao`: Create a DAO.
- `get_dao`: Retrieve a DAO.
- `destroy_dao`: Destroy a DAO.
- `issue_token: Issue a token for a DAO.
- `get_dao_asset_id`: Retrieve the DAO asset ID.
- `set_metadata`: Set metadata for a DAO, this is a web/ipfs link with a hash of the content. See our service for an example.
- `get_metadata`: Load metadata for a DAO.
- `has_hookpoint`: Check if a DAO has a registered hookpoint.
- `get_hookpoint`: Retrieve the hookpoint for a DAO.
- `set_hookpoint`: Set the hookpoint for a DAO, see the hookpoint contract for details.
- `remove_hookpoint`: Remove the hookpoint for a DAO.
- `change_owner`: Transfer ownership of a DAO.
