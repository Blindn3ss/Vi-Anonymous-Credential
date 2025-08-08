# Vi-Anonymous-Credential

This project implements a prototype of an anonymous credential issuance system using a bilinear pairing-based accumulator, designed to support an allow-list mechanism for revoking credentials and preventing misuse.

The accumulator logic is currently being updated based on the model proposed by Flamini et al. (2025).
👉 Read the paper here [https://eprint.iacr.org/2025/549.pdf]

---

## Project Components

### Accumulator  
Represents the core bilinear pairing-based accumulator structure.  
Includes logic for initializing, updating, and testing accumulator behavior via a simple `main` function.

### Credential  
Defines a credential that includes:  
- A BBS+ signature over a list of attribute values (messages).  
- The issuer's public key.  
This object models the credential held by the user.

### Commitment  
Implements Pedersen-style commitments to hide sensitive message values.  
Used during zero-knowledge proof generation to ensure privacy.

### Witness  
Contains the data a credential holder maintains to prove non-revocation:  
- A private scalar `x`  
- A Pedersen commitment `c_x`  
- A trapdoor witness `w_x_t`  
- The credential's BBS+ signature  

---

## How to Test with the Blockchain

1. **Start the Blockchain Server**

   Open a terminal and run:
   ```
   node registry/blockchain_server.js
   ```
   You should see:
   ```
   Blockchain API running on port 3001
   ```

2. **Run the Rust Tests**

   In another terminal, from the project root, run:
   ```
   cargo test --test issue_vc -- --nocapture
   cargo test --test revoke_vc -- --nocapture
   ```
   This will:
   - Issue a credential and log it to the blockchain.
   - Revoke a credential and log the revocation to the blockchain.

3. **Check the Blockchain Server Terminal**

   You will see logs like:
   ```
   [ADD] Block added with data: "CredentialIssued:..."
   [ADD] Block added with data: "CredentialRevoked:..."
   [CHAIN] Chain requested
   [VERIFY] Chain verification: true
   ```

4. **(Optional) Query the Blockchain API**

   You can use `curl` or Postman to check the chain:
   ```
   curl http://localhost:3001/chain
   ```

---

In Progress:
- Integration of the ZKP generation algorithm for the holder to prove validity and non-revocation of their credential.

---
