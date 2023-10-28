# My NFT Project

This project involves creating and deploying an NFT (Non-Fungible Token) contract on the Solana blockchain. Follow the steps below to get started.

## Clone the Repository

To get started, clone this repository to your local machine:

```bash
git clone https://github.com/manthanvw/Solana-Bootcamp-Final-Project.git
```
Navigate to the Program Directory
After cloning the repository, navigate to the "program" directory:

```bash
cd program
```
## Build and Deploy
### Step 1: Build the Contract
Open a terminal window from the terminal tab above and navigate to the generated directory using the command:
```cd program```

Then, build the contract using the following command:
```cargo build-bpf```

### Step 2: Set Your Config File
If there are no errors, set your config file to connect to the Solana devnet by running the following command:
```solana config set --url devnet```

### Step 3: Get Devnet Tokens
To deploy a contract, you'll need some devnet tokens. Obtain devnet tokens by running:
```solana airdrop 1```

You can request as many airdrops as you need. After that, you can check your balance with the command:
```solana balance```

### Step 4: Build and Deploy the Contract
Once you've successfully built the contract, deploy it by running the following command from the generated directory:
```solana program deploy target/deploy/nft.so```

## Frontend Testing
After the deployment is complete, you will receive a program ID. Be sure to copy and save this program ID, as it will be used to configure the client library.
Now, navigate to the "program_client" directory:
```cd ../program_client```

### Step 1: Download Dependencies
To run the code in this directory, you need to install the required dependencies. Open a terminal, navigate to the "program_client" directory (which contains the "package.json" file), and run the following command to install the node_modules dependencies:
```yarn install```

### Step 2: Install SPL Token Library
This contract relies on the "@solana/spl-token" package, which needs to be installed manually. Execute the following command to install it:
```yarn add @solana/spl-token```

### Step 3: Run app.ts
With all the dependencies installed, you're ready to test your NFT contract. Execute the client by running the following command:
```npx ts-node app.ts <YOUR_PROGRAM_ID>```

Replace <YOUR_PROGRAM_ID> with the program ID you obtained during contract deployment.

### Step 4: Explore the Output
In your terminal, you should see an output similar to the one below.
