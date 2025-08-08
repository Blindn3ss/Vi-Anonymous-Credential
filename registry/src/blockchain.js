// This file is now a placeholder for interacting with a Hardhat/Ethereum smart contract.
// Remove the custom blockchain logic.

require('dotenv').config();
const { ethers } = require("ethers");
const fs = require("fs");
const path = require("path");

// Connect to the running Hardhat node
const provider = new ethers.JsonRpcProvider("http://localhost:8545");

// Use the first account's private key (from Hardhat node output)
const FIRST_ACCOUNT_PRIVATE_KEY = process.env.FIRST_ACCOUNT_PRIVATE_KEY; // <-- replace with your actual key
const signer = new ethers.Wallet(FIRST_ACCOUNT_PRIVATE_KEY, provider);

// Load the contract ABI
const abi = JSON.parse(
  fs.readFileSync(
    path.join(__dirname, "../artifacts/contracts/VCRegistry.sol/VCRegistry.json"),
    "utf8"
  )
).abi;

// Get a deployed contract instance
function getVCRegistry() {
  const contractAddress = process.env.VC_REGISTRY_ADDRESS;
  return new ethers.Contract(contractAddress, abi, signer);
}

// Issue a credential
async function issueCredential(credId, to) {
  const registry = getVCRegistry();
  const tx = await registry.issueCredential(credId, to);
  await tx.wait();
  return tx.hash;
}

// Revoke a credential
async function revokeCredential(credId) {
  const registry = getVCRegistry();
  const tx = await registry.revokeCredential(credId);
  await tx.wait();
  return tx.hash;
}

module.exports = {
  getVCRegistry,
  issueCredential,
  revokeCredential,
};