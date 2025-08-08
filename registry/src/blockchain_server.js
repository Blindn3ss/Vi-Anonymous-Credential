const express = require('express');
const { issueCredential, revokeCredential } = require('./blockchain');
const app = express();
app.use(express.json());

// Issue a credential (add block)
app.post('/add', async (req, res) => {
  const { credId, to } = req.body;
  try {
    const txHash = await issueCredential(credId, to);
    console.log(`[ADD] Credential issued: ${credId} to ${to}, tx: ${txHash}`);
    res.json({ message: 'Credential issued', txHash });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Failed to issue credential' });
  }
});

// Revoke a credential
app.post('/revoke', async (req, res) => {
  const { credId } = req.body;
  try {
    const txHash = await revokeCredential(credId);
    console.log(`[REVOKE] Credential revoked: ${credId}, tx: ${txHash}`);
    res.json({ message: 'Credential revoked', txHash });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'Failed to revoke credential' });
  }
});

app.listen(3001, () => console.log('Blockchain API running on port 3001 (Hardhat mode)'));
