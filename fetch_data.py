import os
import json
from web3 import Web3
from hexbytes import HexBytes
import hashlib

# Fetch RPC URL from environment variable
rpc_url = os.getenv("OPTIMISM_RPC_URL")
if not rpc_url:
    print("OPTIMISM_RPC_URL not set")
    os._exit(1)

# Initialize Web3
w3 = Web3(Web3.HTTPProvider(rpc_url))

# Constants
block_number = 121184863
output_root = "3ea8b0e09b39e9daa1b1520fe59faef02de3656d230d876544952cbc44d6d71f"
L2_TO_L1_BRIDGE = "0x4200000000000000000000000000000000000016"
storage_keys = ["0x0", "0x1"]


def hexbytes_to_str(data):
    if isinstance(data, int):
        return hex(data)[2:]  # Convert integer to hex without '0x' prefix
    hex_str = data.hex() if isinstance(data, (bytes, HexBytes)) else data
    return hex_str[2:] if hex_str.startswith("0x") else hex_str


def concatenate_hex_strings(hex_strings):
    return ''.join(hex_strings)


# Initialize trie data with output root directly as a key
trie_data = {output_root: output_root}

# Step 1: Fetch and format the account proof
account_proof = w3.eth.get_proof(L2_TO_L1_BRIDGE, [], block_number)
hashed_address = hashlib.sha256(L2_TO_L1_BRIDGE.encode()).hexdigest()

# Convert each part of the account data to hex without '0x' prefix
balance_hex = hexbytes_to_str(account_proof.balance)
nonce_hex = hexbytes_to_str(account_proof.nonce)
code_hash_hex = hexbytes_to_str(account_proof.codeHash)
storage_hash_hex = hexbytes_to_str(account_proof.storageHash)

# Concatenate all account data
concatenated_account_proof = concatenate_hex_strings(
    [balance_hex, nonce_hex, code_hash_hex, storage_hash_hex] +
    [hexbytes_to_str(proof) for proof in account_proof.accountProof]
)

# Store the concatenated account proof data under the hashed address key
trie_data[hashed_address] = concatenated_account_proof

# Step 2: Fetch and format storage proofs
for key in storage_keys:
    storage_proof = w3.eth.get_proof(L2_TO_L1_BRIDGE, [key], block_number)
    key_hash = hashlib.sha256(HexBytes(key)).hexdigest()

    # Concatenate all proofs within each storage proof
    concatenated_storage_proof = concatenate_hex_strings(
        [hexbytes_to_str(proof_item) for proof in storage_proof.storageProof for proof_item in proof.proof]
    )

    # Store the concatenated storage proof data under the hashed key
    trie_data[key_hash] = concatenated_storage_proof

# Output path for output.json in the testdata folder
output_path = f"crates/executor/testdata/block_{block_number}_exec/output.json"
os.makedirs(os.path.dirname(output_path), exist_ok=True)

# Write trie data to output.json
with open(output_path, "w") as f:
    json.dump(trie_data, f, indent=2)

print(f"Data saved to {output_path}")
