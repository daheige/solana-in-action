# first rust solana
Rust is the most common programming language to write Solana programs with. This quickstart guide will demonstrate how to quickly setup, build, and deploy your first Rust based Solana program to the blockchain.

# rust learning
- https://www.rust-lang.org/zh-CN/learn
- https://doc.rust-lang.org/book/
- https://kaisery.github.io/trpl-zh-cn/

# install rust
https://www.rust-lang.org/zh-CN/tools/install
Run the following command to install rust:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then add env to ~/.bash_profile
```shell
. "$HOME/.cargo/env"
```

# run you localhost validator
the solana comme with the test validator built in.This command line tool will allow you to run a full blockchain cluster on your machine.
```shell
solana-test-validator
```
Configure your Solana CLI to use your localhost validator for all your future terminal commands and Solana program deployment:
```shell
solana config set --url localhost
```
output:
```
Config File: $HOME/.config/solana/cli/config.yml
RPC URL: http://localhost:8899
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: $HOME/.config/solana/id.json
Commitment: confirmed
```

# Create a new Rust library with Cargo 
Solana programs written in Rust are libraries which are compiled to BPF bytecode and saved in the .so format.
Initialize a new Rust library named hello_world via the Cargo command line:
```shell
cargo new hello_world --lib
cd hello_world
```
add the solana-program crate to your new rust library:
```shell
cargo add solana-program
# It is highly recommended to keep your solana-program and other Solana Rust dependencies in-line with your installed version of the Solana CLI. 
# or run: cargo add solane-program@"=2.0.10"
```
then open your `Cargo.toml` file and add these required Rust library configuration settings.
```toml
[lib]
name = "hello_world"
crate-type = ["cdylib", "lib"]
```

# Create your first solana program
At the top of `src/lib.rs`,import `solana-program` crate and bring your needed items into the local namespace:
```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
```
Every Solana program must define an `entrypoint` that tells the Solana runtime where to start executing your onchain code. Your program's entrypoint should provide a public function named `process_instruction`:
```rust
entrypoint!(process_instruction);

// ProgramResult in solana define:
// pub type ProgramResult = ResultGeneric<(), ProgramError>;
// define process_instruction func
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("hello,world!");

    Ok(())
}
```
Every onchain program should return the `Ok`result enum with a value of `()`. This tells the Solana runtime that your program executed successfully without errors.

This program above will simply log a message of "Hello, world!" to the blockchain cluster, then gracefully exit with `Ok(())`.

# Build your Rust program
```shell
cargo build-sbf
```
After each time you build your Solana program, the above command will output the build path of your compiled program's .so file and the default keyfile that will be used for the program's address. cargo build-sbf installs the toolchain from the currently installed solana CLI tools. You may need to upgrade those tools if you encounter any version incompatibilities. In case you get an error like:
xxx.so.you may need to go to `~/.cache/solana/` and `rm -rf` the platform tools there and then run `cargo build-sbf` again.

# Deploy your Solana program
Using the Solana CLI, you can deploy your program to your currently selected cluster:
```shell
solana program deploy ./target/deploy/hello_world.so
```
Once your Solana program has been deployed (and the transaction finalized), the above command will output your program's public address (aka its "program id").
```
# example output
Program Id: xxx
```
Congratulations! You have successfully setup, built, and deployed a Solana program using the Rust language.

# call the hello_world program
1. install nodejs and view nodejs version
2. create the client file
Install the Solana web3.js library and the Solana helpers library:
```shell
npm install @solana/web3.js @solana-developers/helpers
```
Create a new file called client.mjs and add the following code:
```js
import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
 
const programId = new PublicKey("YOUR_PROGRAM_ID");
 
// Connect to a solana cluster. Either to your local test validator or to devnet
const connection = new Connection("http://localhost:8899", "confirmed");
//const connection = new Connection("https://api.devnet.solana.com", "confirmed");
 
// We load the keypair that we created in a previous step
const keyPair = await getKeypairFromFile("~/.config/solana/id.json");
 
// Every transaction requires a blockhash
const blockhashInfo = await connection.getLatestBlockhash();
 
// Create a new transaction
const tx = new Transaction({
  ...blockhashInfo,
});
 
// Add our Hello World instruction
tx.add(
  new TransactionInstruction({
    programId: programId,
    keys: [],
    data: Buffer.from([]),
  }),
);
 
// Sign the transaction with your previously created keypair
tx.sign(keyPair);
 
// Send the transaction to the Solana network
const txHash = await connection.sendRawTransaction(tx.serialize());
 
console.log("Transaction sent with hash:", txHash);
 
await connection.confirmTransaction({
  blockhash: blockhashInfo.blockhash,
  lastValidBlockHeight: blockhashInfo.lastValidBlockHeight,
  signature: txHash,
});
 
console.log(
  `Congratulations! Look at your ‘Hello World’ transaction in the Solana Explorer:
  https://explorer.solana.com/tx/${txHash}?cluster=custom`,
);
```
3. Run the client 
```shell
node client.mjs
```
You should see the following output:
```
Congratulations! Look at your ‘Hello World’ transaction in the Solana Explorer:
  https://explorer.solana.com/tx/xxx?cluster=custom
```

# Deploy to Solana devnet 
Now you have successfully deployed your program to your local cluster. If you want to deploy it to the public devnet to show your program to your friends you can do so by running the following command:
```shell
solana program deploy ./target/deploy/hello_world.so --url https://api.devnet.solana.com
```
Then change the connections url in your client.mjs also to https://api.devnet.solana.com and run the client again.
```shell
node client.mjs
```
You should see the same output as before but now on the public devnet cluster. You can see the transaction in the Solana Explorer again. Now you just need to switch it to devnet on the top right.

Congratulations, now everyone in the world can see your "Hello World" transaction on the Solana blockchain.

# next steps
See the links below to learn more about writing Rust based Solana programs:
- https://solana.com/docs/programs/overview
- https://solana.com/docs/intro/quick-start
- https://solana.com/docs/programs/lang-rust
- https://solana.com/docs/programs/debugging
