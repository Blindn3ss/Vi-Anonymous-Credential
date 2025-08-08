# How to Test with the Blockchain

1. **Start the Blockchain Server**

   Open a terminal in the `registry` folder and run:
   ```
   node blockchain_server.js
   ```
   You should see:
   ```
   Blockchain API running on port 3001
   ```

2. **Run the Rust Tests**

   In a separate terminal, from the project root, run:
   ```
   cargo test --test issue_vc -- --nocapture
   cargo test --test revoke_vc -- --nocapture
   ```
   This will:
   - Issue a credential and add it to the blockchain.
   - Revoke a credential and add the revocation to the blockchain.

3. **Check Blockchain Server Output**

   The blockchain server terminal will show logs like:
   ```
   [ADD] Block added with data: "CredentialIssued:..."
   [ADD] Block added with data: "CredentialRevoked:..."
   [CHAIN] Chain requested
   [VERIFY] Chain verification: true
   ```

4. **(Optional) Query the Blockchain API**

   You can check the blockchain state using:
   ```
   curl http://localhost:3001/chain
   ```

**Note:**  
- The blockchain server must be running before you run the Rust tests.
- Each test run will add new blocks to the blockchain.

# How to Use Smart Contracts with Hardhat

If you want to use smart contracts for VC issuance and revocation, you can use [Hardhat](https://hardhat.org/) to develop, deploy, and test contracts on Ethereum-compatible blockchains.

## 1. Install Hardhat

In your project or a new directory:

# Hardhat Setup Guide

Follow these steps to set up Hardhat for smart contract development and testing:

## 1. Initialize Hardhat in the `registry` Folder

Open a terminal in the `registry` directory and run:
```
npx hardhat
```
- Choose "Create a basic sample project" (or "Create an empty hardhat.config.js" if you prefer).
- Follow the prompts and install any suggested dependencies.

## 2. Install Hardhat Toolbox (Optional but Recommended)

```
npm install --save-dev @nomicfoundation/hardhat-toolbox
```

## 3. Write Your Smart Contract

Create a new file, for example:  
`contracts/VCRegistry.sol`
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract VCRegistry {
    event CredentialIssued(bytes32 indexed credId, address indexed to);
    event CredentialRevoked(bytes32 indexed credId);

    mapping(bytes32 => bool) public issued;
    mapping(bytes32 => bool) public revoked;

    function issueCredential(bytes32 credId, address to) public {
        require(!issued[credId], "Already issued");
        issued[credId] = true;
        emit CredentialIssued(credId, to);
    }

    function revokeCredential(bytes32 credId) public {
        require(issued[credId], "Not issued");
        revoked[credId] = true;
        emit CredentialRevoked(credId);
    }

    function isRevoked(bytes32 credId) public view returns (bool) {
        return revoked[credId];
    }
}
```

## 4. Compile the Contract

```
npx hardhat compile
```

## 5. Start a Local Hardhat Node

```
npx hardhat node
```

## 6. Deploy the Contract

Create a deploy script, e.g. `scripts/deploy.js`:
```js
// scripts/deploy.js
async function main() {
  const VCRegistry = await ethers.getContractFactory("VCRegistry");
  const registry = await VCRegistry.deploy();
  await registry.deployed();
  console.log("VCRegistry deployed to:", registry.address);
}
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```
Run the deploy script:
```
npx hardhat run scripts/deploy.js --network localhost
```
- Note the deployed contract address and set it as the `VC_REGISTRY_ADDRESS` environment variable for your Node.js server.

## 7. Update Your Node.js Server

- Make sure your Node.js API uses the correct contract address and is running while the Hardhat node is running.

---

**Now you are ready to interact with your smart contract from Node.js or Rust!**
