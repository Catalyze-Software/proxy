#!/bin/bash

# Convert the Candid output to JSON
dfx canister call proxy --output json --ic mig_profiles_get_all > scripts/profiles.json

# Count the number of entries in the JSON file
entry_count=$(jq '. | length' scripts/profiles.json)

# Print the number of entries
echo "Number of entries: $entry_count"