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
