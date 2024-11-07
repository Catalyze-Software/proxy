#!/bin/bash

# Define the amount in e8s format
amount="15000000000: u64"

# Extract all principals from the JSON file
principals=$(jq -r '.[]."0"' scripts/profiles.json)

# Format the principals into a Candid vec of Receiver records
candid_vec="vec {"
for principal in $principals; do
    candid_vec+="record { principal = \"$principal\"; amount = $amount; }, "
done
# Remove the trailing comma and space, then close the vec
candid_vec=${candid_vec%, }"}"

# Write the Candid vec to a file
echo "$candid_vec" > scripts/principals.candid

# Print a message indicating success
echo "Candid vec written to scripts/principals.candid"
