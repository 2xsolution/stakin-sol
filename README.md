# MANGO

## Content

- [deployment](#deployment)

- [functionalities](#functionalities)

- [Accounts](#accounts)

### Devnet Deployment

`Anchor.toml`

Ensure that

1. `cluster = "devnet"`
2. `wallet = "~/.config/solana/id.json"` is the location of your solana key pair
3. `[programs.devnet]`

```toml
[toolchain]

[features]
seeds = false
skip-lint = false

[programs.devnet]
mango = "FnPEsqZTAgJpbYmNKYTyWY6NaHm6yZ1Z2yXjXkMFjU6f"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

`CLI`

1. Open your terminal
2. Run this command to set solana configuration

```bash
solana config set --keypair ~/.config/solana/id.json  --url devnet
```

3. Run this command to check configuration

```bash
solana config get  
```

4. Run the command to view address

```bash
solana address
```

4. Run the command to get testnet SOL

```bash
solana airdrop 5
```

5. Run the command to deploy to testnet

```bash
anchor deploy 
```

Command above should return success with a Program Id.

6. View deployed contract using this command

```bash
solana program show <Program Id>
```

The command above should provide relevant details of the deployed contract

Modify `program/src/lib.rs`

```rust
declare_id!("FnPEsqZTAgJpbYmNKYTyWY6NaHm6yZ1Z2yXjXkMFjU6f");
```

to

```rust
declare_id!("<Program Id>")
```

Modify `Anchor.toml`

```toml
[programs.devnet]
mango = "FnPEsqZTAgJpbYmNKYTyWY6NaHm6yZ1Z2yXjXkMFjU6f"
```

to

```toml
[programs.devnet]
mango = "<Program Id>"
```

### Mainet Deployment

`Anchor.toml`

Ensure that

1. `cluster = "mainnet"`
2. `wallet = "~/.config/solana/id.json"` is the location of your solana key pair
3. `[programs.mainnet]`

```toml
[toolchain]

[features]
seeds = false
skip-lint = false

[programs.mainnet]
mango = "FnPEsqZTAgJpbYmNKYTyWY6NaHm6yZ1Z2yXjXkMFjU6f"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "mainnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

`CLI`

1. Open your terminal
2. Get Mainnet account keypair from wallet eg. Phantom and save in  `~/.config/solana/<YOUR_ACCOUNT_KEYPAIR>.json`

3. Run this command to set solana configuration to mainnet

```bash
solana config set --keypair ~/.config/solana/<YOUR_ACCOUNT_KEYPAIR>.json  --url mainnet
```

4. Run this command to check configuration

```bash
solana config get  
```

5. Run the command to view address

```bash
solana address
```

6. Fund address with  >5 SOL

7. Get authority(admin) account ready and copy address

8. Run the command to deploy to mainnet

```bash
anchor deploy 
```

### Functionalities

- `stake`
- `unstake`
- `addReferral`
- `claimReward`

