# solana-reward-contract
This repo contains smart contract that allows users to earn rewards (SOL) by selecting an activity from a predefined list. The rewards are dynamically based on the total number of users and available tasks using a demand-supply logic


# Pre-requisites
## Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustc --version
```
## Install Solana CLI
```
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
solana --version
```

## Install AVM
```
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm --version
avm install 0.30.1
avm use 0.30.1
anchor --version
```

## Setup project
```
anchor init rewards
cd rewards

anchor build
anchor test
```