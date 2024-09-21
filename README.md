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

4. view the solana version
```shell
solana --version
solana-cli 1.18.24 (src:6b04e881; feat:3241752014, client:Agave)
```
# solana development
Solana's development can be divided into two main parts:
- On-chain program development(链上程序开发）: This is where you create and deploy custom programs directly to the blockchain. Once deployed, they are available to anyone who knows how to communicate with them. You can write these programs in Rust, C, or C++. Rust is currently the most supportive of on-chain application development.
create a framework for client development
- Client development（客户端开发）: This is the part that writes software (called a decentralized application or dApp) that communicates with on-chain programs. Your application can submit transactions to perform actions on the chain. Client development can be written in any programming language.
Client development currently supports languages such as Rust, Typescript, Python, Java, C++, GO, Kotlin, Dart, etc. I recommend that you use Rust, GO, and Nodejs for client development, mainly because these three languages are rich in ecology, provide more libraries and tools, and more developers, and the cost of development is relatively low.

# Client development demo
First make sure you have the nodejs software installed, then execute the following command to create a client development framework application.
```shell
mkdir -p ~/web/rust/solana
cd solana
npx create-solana-dapp helloworld
```
output:
```
Need to install the following packages:
create-solana-dapp@3.1.0
Ok to proceed? (y) y

┌  create-solana-dapp 3.1.0
│
◆  Project name: helloworld
│
◇  Select a preset
│  React + React Router DOM
│
◇  Select a UI library
│  None
│
◇  Select an Anchor template
│  Anchor Counter program with tests
│
◆  Anchor program name: helloworld
│
◇  Successfully created workspace with npm.
│
◒  Installing preset @solana-developers/preset-react@3.1.0...bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
◒  Installing preset @solana-developers/preset-react@3.1.0Fetching @nx/rollup...
◇  Successfully installed preset @solana-developers/preset-react@3.1.0.
│
◇  Installation successful! ──────────────────────────────╮
│                                                         │
│  That's it!                                             │
│                                                         │
│  Change to your new directory and start developing:     │
│                                                         │
│  cd ./helloworld                                        │
│                                                         │
│  Start the React app:                                   │
│                                                         │
│  npm run dev                                            │
│                                                         │
│  Run Anchor commands:                                   │
│                                                         │
│  npm run anchor build | test | localnet | deploy        │
│                                                         │
│  Generate more features using the following command:    │
│                                                         │
│  npm run feature                                        │
│                                                         │
│  Could not find Anchor version. Please install Anchor.  │
│                                                         │
│  https://www.anchor-lang.com/docs/installation          │
│                                                         │
├─────────────────────────────────────────────────────────╯
│
└  Good luck with your project!
```
This will create a new project with all the necessary files and basic configuration to start building on Solana. The scaffolding will include a sample front end and an on-chain program template if you choose.
Here I chose React + React Router DOM.

create-solana-dapp docs:
- https://github.com/solana-developers/create-solana-dapp?tab=readme-ov-file#create-solana-dapp

# docs
- https://www.solana-cn.com/SolanaDocumention/home.html
- https://www.solana-cn.com/SolanaBasic/000.html
