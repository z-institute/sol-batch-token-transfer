# Sol Batch Token Transfer CSV

## Send test SOL to Z Vault
```
solana transfer 7PDqBQdPMZYiF2iS9ANbc1qYXL2QMkNv6aW9STgu8iNb 0.5 --allow-unfunded-recipient --url https://rpc-mainnet-fork.dappio.xyz

solana balance 7PDqBQdPMZYiF2iS9ANbc1qYXL2QMkNv6aW9STgu8iNb --url https://rpc-mainnet-fork.dappio.xyz

solana balance HXcdCwwu1wkS882Gs8rRV6f83MyestRyB5HmWGwuiFiq --url https://rpc-mainnet-fork.dappio.xyz
```

## Running the Project
1. You have a SPL token
2. Create `data.csv` referencing `data.csv.example`
3. Create `.env` referencing `.env.example`
3. `cargo run` to batch transfer token
4. Result will be saved in `done.csv`