# Ana-Chain

Ana-Chain is a fullstack social application using Solana as a backend for decentralized governance.

---

### **Setup Commands Using `pnpm`**

Once the repository has been cloned, you can use `pnpm` commands to streamline development and build tasks. `pnpm` is a fast and efficient package manager, ideal for managing dependencies in a complex project.

Here is the list of key commands:

---

### **Global Commands**

#### **`pnpm install`**
- Installs all the dependencies required for the project.
- This includes both frontend (`web`) and backend (`anchor`) dependencies, preparing the entire project in one step.
- **Note:** This process may take some time as it installs all dependencies for the fullstack project. You can monitor the progress in your terminal.

#### **`pnpm dev`**
- Starts the development server for the frontend React application with live reload enabled.
- Ideal for fast-paced development and real-time previews.

#### **`pnpm build`**
- Compiles and builds the frontend application for production. The final files are placed in the `out` directory.

#### **`pnpm start`**
- Runs the production version of the frontend application, perfect for local verification before deployment.

---

### **Anchor-Related Commands**

#### **`pnpm anchor`**
- Allows you to run any Anchor CLI command directly from the root directory. Equivalent to navigating to the `anchor` directory and running the Anchor command.

Example:
```bash
pnpm anchor build
```

#### **`pnpm anchor:build`**
- Compiles the Solana program in the `anchor` directory using `anchor build`.
- Useful if you want to build your smart contract manually without running the entire dev flow.

#### **`pnpm anchor-localnet`**
- Starts a local Solana validator and deploys the Anchor program to it.
- This is crucial for testing programs locally without relying on external networks.

#### **`pnpm anchor-test`**
- Runs tests for the Solana program using Anchor's built-in test framework.

#### **`pnpm anchor-deploy`**
- Deploys the current program onto the specified Solana blockchain configured in the `Anchor.toml` file.

---

### **Miscellaneous Commands**

#### **`pnpm lint-init`**
- Initializes ESLint for the project setup.

#### **`pnpm lint`**
- Runs ESLint and fixes style issues in `.js`, `.ts`, and `.tsx` files.

---

### **Project-Specific Dependency Installation**

#### **Frontend (governance-ui)**
To install dependencies specific to the frontend located in the `governance-ui` folder, use the following command:

```bash
pnpm gov-ui-install
```

This command navigates into the `governance-ui` directory and installs all its necessary dependencies using `pnpm install`.

---

### **Quick Instructions**

#### Install dependencies:
```bash
pnpm install
```

#### Start the project in development mode:
```bash
pnpm dev
```

#### Compile the Anchor program:
```bash
pnpm anchor:build
```

#### Start a local Solana network:
```bash
pnpm anchor-localnet
```

---

## **Available Applications**

### **Anchor**
- This is a Solana program written in Rust using the Anchor framework.
- You can run all standard Anchor commands by either navigating to the `anchor` directory or prefixing your Anchor commands with `pnpm`.

Example:
```bash
pnpm anchor build
```