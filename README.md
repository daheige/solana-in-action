# solana-in-action
solana in Action

# install
- https://solana.com/zh/docs/intro/installation
1. install rust
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```
then add env to ~/.bash_profile
```shell
. "$HOME/.cargo/env"
```

2. install solana
```shell
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```
3. add env
```shell
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```
source ~/.bash_profile

# docs
- https://www.solana-cn.com/SolanaDocumention/home.html
- https://www.solana-cn.com/SolanaBasic/000.html
