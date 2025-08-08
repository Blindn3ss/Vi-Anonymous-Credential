use ark_bls12_381::{Bls12_381, Fr};
use bbs_plus::prelude::{SignatureParamsG1, SecretKey, SignatureG1, PublicKeyG2};
use ark_ff::UniformRand;
use sha2::{Sha256, Digest};
use rand::thread_rng;
use BMGen::credential::Credential;
use ark_serialize::CanonicalSerialize;
use hex::encode;

#[test]
fn test_issue_second_vc_to_another_holder_and_blockchain() {
    let mut rng = thread_rng();

    // Issuer generates secret key (can be the same as previous test)
    let seed = [1u8; 32]; // Different seed for demonstration
    let sk = SecretKey::<Fr>::generate_using_seed::<sha2::Sha256>(&seed);

    // Issuer prepares signature parameters
    let message_count = 2;
    let params = SignatureParamsG1::<Bls12_381>::generate_using_rng(&mut rng, message_count);

    // Holder's messages (attributes) - different from the first holder
    let fr_messages: Vec<Fr> = (0..message_count).map(|_| Fr::rand(&mut rng)).collect();

    // Issuer creates BBS+ signature over holder's messages
    let signature = SignatureG1::new(&mut rng, &fr_messages, &sk, &params)
        .expect("Signature creation should succeed");

    // Issuer constructs Credential object
    let credential = Credential {
        signature: signature.clone(),
        messages: fr_messages.clone(),
        issuer_pk: None,
    };

    // Derive a unique credId as a 32-byte hex string from the signature hash
    let mut sig_bytes = vec![];
    signature.serialize_compressed(&mut sig_bytes).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&sig_bytes);
    let hash = hasher.finalize();
    let cred_id = format!("0x{}", encode(hash)); // 32-byte hex string

    // Placeholder Ethereum address for the second holder
    let to_address = "0x000000000000000000000000000000000000beef";

    // --- Blockchain interaction (Hardhat/Node.js API) ---
    let client = reqwest::blocking::Client::new();
    let api_url = "http://localhost:3001";

    // Add a block for the issued credential using the new API
    let resp = client.post(&format!("{}/add", api_url))
        .json(&serde_json::json!({ "credId": cred_id, "to": to_address }))
        .send()
        .expect("Failed to add block");
    assert!(resp.status().is_success());

    // --- Additional: Verify the signature is valid ---
    let pk = PublicKeyG2::generate_using_secret_key(&sk, &params);
    let verified = signature.verify(&fr_messages, pk, params.clone()).is_ok();
    assert!(verified, "Signature should verify for issued messages and params");

    println!("Second credential issued to another holder: {:?}", credential.messages);
    println!("Blockchain tx response: {:?}", resp.text().unwrap());
}
