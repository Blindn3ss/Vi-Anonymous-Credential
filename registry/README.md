# How to Set Up and Test the Blockchain Integration

## 1. Prerequisites

- Node.js and npm installed
- Rust and Cargo installed
- Hardhat and dependencies installed in the `registry` folder

## 2. Environment Setup

1. **Start the Hardhat Node**

   In the `registry` folder:
   ```
   npx hardhat node
   ```

2. **Deploy the Smart Contract**

   In a new terminal (also in `registry`):
   ```
   npx hardhat run scripts/deploy.js --network localhost
   ```
   - Copy the deployed contract address from the output.

3. **Configure Environment Variables**

   Create a `.env` file in the `registry` folder with:
   ```
   VC_REGISTRY_ADDRESS=0xYourDeployedContractAddress
   FIRST_ACCOUNT_PRIVATE_KEY=0xYourHardhatAccountPrivateKey
   ```
   - Use the contract address from the deploy step.
   - Use the private key of the first account shown when you start the Hardhat node.

## 3. Start the Blockchain API Server

In the `registry` folder:
```
node src/blockchain_server.js
```
You should see:
```
Blockchain API running on port 3001 (Hardhat mode)
```

## 4. Run the Rust Tests

In the project root:
```
cargo test --test issue_vc_holder -- --nocapture
cargo test --test issue_vc_holder_2 -- --nocapture
```
- These will issue credentials to different holders and interact with the blockchain via the Node.js API.

## 5. Check Logs

- The Node.js server terminal will show logs for each credential issued or revoked.
- The Hardhat node terminal will show transaction logs for contract deployments and interactions.

## 6. (Optional) Query the Blockchain API

You can check the blockchain state or test endpoints using:
```
curl -X POST http://localhost:3001/add -H "Content-Type: application/json" -d "{\"credId\":\"0x...\",\"to\":\"0x...\"}"
```
or similar requests.

---

**Notes:**
- Always start the Hardhat node and deploy the contract before running the server or tests.
- Update `.env` with the correct contract address and private key after each deployment or node restart.
- The Rust tests and Node.js server interact with the smart contract for VC issuance and revocation.
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
