# CREATE TOKEN USING CLI ON SOLANA

#### Prerequisites:
1. Install Rust from https://www.rust-lang.org/tools/install
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools

- I am doing on oracle VM virtual box with ubuntu.

- For install oracle VM virtual box and run with ubuntu, I think this video will provide help: https://www.youtube.com/watch?v=x5MhydijWmc&t=252s\

- After setting all requirements, lets go to create the token:

#### CREATION:

1. Create the folder name solanaBlockchain and then cd solanaBlockchain.
2. Create the wallet, using this command:
```
solana-keygen new
```
then press enter and your wallet public key will be appear.
3. For get the public key, using:
```
solana-keygen pubkey
```
4. For check the balance of your wallet on devnet:
```
solana balance --url devnet
```
5. For get the 2 sol in your wallet, using:
```
solana airdrop 2 your-public-key --url devnet
```
6. Install libudev-dev:
```
sudo apt-get install libudev-dev
```
7. For create the token, I am using spl-token-cli library:
```
cargo install spl-token-cli
```
8. For create token:
```
spl-token create-token --url devnet
```
9. Create account for token:
```
spl-token create-account token-address-those-provided-after-run-8-command --url devnet
```
10. For check the token balance:
```
spl-token balance token-address-those-provided-after-run-8-command --url devnet
```
11. For mint 100 tokens:
```
spl-token mint token-address-those-provided-after-run-8-command 100 --url devnet
```
12. For check the token supply:
```
spl-token supply token-address-those-provided-after-run-8-command --url devnet
```
13. For disable the minting of tokens:
```
spl-token authorize token-address-those-provided-after-run-8-command mint --disable --url devnet
```
now you can't mint more tokens.
14. For burn 10 tokens:
```
spl-token burn your-acount-address-in-which-token-are-exist 10 --url devnet
```
15. Transfer 30 tokens:
```
spl-token transfer token-address-those-provided-after-run-8-command 30 recepient-address --url devnet --allow-unfunded-recipient --fund-recipient
```

### Completed