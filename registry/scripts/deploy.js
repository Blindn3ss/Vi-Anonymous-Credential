async function main() {
  const VCRegistry = await ethers.getContractFactory("VCRegistry");
  const registry = await VCRegistry.deploy();
  // Print both .target and .address for compatibility
  console.log("VCRegistry deployed to:", registry.target ? registry.target : registry.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
