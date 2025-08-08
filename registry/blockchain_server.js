const express = require('express');
const { Blockchain } = require('./blockchain');
const app = express();
app.use(express.json());

const chain = new Blockchain();

app.post('/add', (req, res) => {
  const { data } = req.body;
  chain.addBlock(data);
  console.log(`[ADD] Block added with data: ${JSON.stringify(data)}`);
  res.json({ message: 'Block added' });
});

app.get('/chain', (req, res) => {
  console.log(`[CHAIN] Chain requested`);
  res.json(chain.chain);
});

app.get('/verify', (req, res) => {
  const valid = chain.verifyChain();
  console.log(`[VERIFY] Chain verification: ${valid}`);
  res.json({ valid });
});

app.listen(3001, () => console.log('Blockchain API running on port 3001'));
