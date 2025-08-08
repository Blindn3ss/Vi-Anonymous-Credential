const crypto = require('crypto');

class Block {
  constructor(index, timestamp, data, prevHash) {
    this.index = index;
    this.timestamp = timestamp;
    this.data = data;
    this.prevHash = prevHash;
    this.hash = this.computeHash();
  }

  computeHash() {
    const str = this.index + this.timestamp + this.data + this.prevHash;
    return crypto.createHash('sha256').update(str).digest('hex');
  }
}

class Blockchain {
  constructor() {
    this.chain = [new Block(0, Date.now(), 'genesis', '0')];
  }

  addBlock(data) {
    const prev = this.chain[this.chain.length - 1];
    const block = new Block(
      prev.index + 1,
      Date.now(),
      data,
      prev.hash
    );
    this.chain.push(block);
  }

  verifyChain() {
    for (let i = 1; i < this.chain.length; i++) {
      const prev = this.chain[i - 1];
      const curr = this.chain[i];
      if (curr.prevHash !== prev.hash || curr.hash !== curr.computeHash()) {
        return false;
      }
    }
    return true;
  }

  getLatest() {
    return this.chain[this.chain.length - 1];
  }

  findByData(data) {
    return this.chain.find(b => b.data === data);
  }
}

module.exports = { Block, Blockchain };
