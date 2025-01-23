# ana-chain

## Getting Started

### Prerequisites

- Node v18.18.0 or higher

- Rust v1.77.2 or higher
- Anchor CLI 0.30.1 or higher
- Solana CLI 1.18.17 or higher

### Installation

#### Clone the repo

```shell
git clone <repo-url>
cd <repo-name>
```
#### Setup Commands Using `pnpm`

Once the repository is cloned, you can take advantage of `pnpm` commands to streamline your development and build tasks. `pnpm` is a fast and efficient package manager that handles dependencies for your project and simplifies various operations.

Here are the key commands available:

- **`pnpm install`**:  
  Installs all the required dependencies for the project. 
- This command covers both the frontend (`web`) and backend (`anchor`) dependencies, 
- essentially preparing the whole build in one step.  
  **Note**: This process may take some time, as it installs all dependencies for the fullstack project. 
- You can monitor the progress displayed in the terminal, 
- so don't worry if it seems like it's taking too long!

- **`pnpm dev`**:  
  Starts the development server for the frontend React application, 
- enabling live preview and rapid development.

- **`pnpm anchor:build`**:  
  Compiles the Solana program in the `anchor` directory using `anchor build`. 
- Useful if you want to build your smart contract manually without running the full development flow.

- **`pnpm anchor keys sync`**:  
  Synchronizes the program ID for the Solana program. This creates a new keypair, 
- updates the Anchor configuration, and declares the program ID in the Rust source file (`declare_id!`). 
- Make sure to manually update the relevant constants in `counter-exports.ts`.

- **`pnpm anchor-localnet`**:  
  Starts a local Solana test validator and deploys the program. 
- A great way to test the program locally without external dependencies.

- **`pnpm anchor-test`**:  
  Runs the tests for the Solana program using the test framework provided by Anchor.

- **`pnpm build`**:  
  Builds the frontend web application for production. Outputs the compiled app to the `out` directory.

- **`pnpm start`**:  
  Runs the production version of the web application, ideal for verifying production builds locally.

By combining these commands with the provided `Dockerfile`,
you can build, test, and run the full application stack (frontend and Solana backend) quickly and effectively.

#### Install Dependencies

```shell
pnpm install
```

#### Start the web app

```
pnpm dev
```

## Apps

### anchor

This is a Solana program written in Rust using the Anchor framework.

#### Commands

You can use any normal anchor commands. Either move to the `anchor` directory and run the `anchor` command or prefix the command with `pnpm`, eg: `pnpm anchor`.

#### Sync the program id:

Running this command will create a new keypair in the `anchor/target/deploy` directory and save the address to the Anchor config file and update the `declare_id!` macro in the `./src/lib.rs` file of the program.

You will manually need to update the constant in `anchor/lib/counter-exports.ts` to match the new program id.

```shell
pnpm anchor keys sync
```

#### Build the program:

```shell
pnpm anchor:build
```

#### Start the test validator with the program deployed:

```shell
pnpm anchor-localnet
```

#### Run the tests

```shell
pnpm anchor-test
```

#### Deploy to Devnet

```shell
pnpm anchor deploy --provider.cluster devnet
```

### web

This is a React app that uses the Anchor generated client to interact with the Solana program.

#### Commands

Start the web app

```shell
pnpm dev
```

Build the web app

```shell
pnpm build
```
