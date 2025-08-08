use ark_bls12_381::{Bls12_381, Fr};
use bbs_plus::prelude::{SignatureParamsG1, SecretKey, SignatureG1, PublicKeyG2};
use ark_ff::UniformRand;
use sha2::Sha256;
use rand::thread_rng;
use BMGen::credential::Credential; // <-- Use your crate name

#[test]
fn test_issuer_issues_vc_to_holder() {
    let mut rng = thread_rng();

    // Issuer generates secret key
    let seed = [0u8; 32];
    let sk = SecretKey::<Fr>::generate_using_seed::<Sha256>(&seed);

    // Issuer prepares signature parameters
    let message_count = 2;
    let params = SignatureParamsG1::<Bls12_381>::generate_using_rng(&mut rng, message_count);

    // Holder's messages (attributes)
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

    // Check that the credential contains the correct signature and messages
    assert_eq!(credential.messages.len(), message_count as usize);
    // Optionally, check signature type
    let sig_type = std::any::type_name_of_val(&credential.signature);
    assert!(sig_type.contains("SignatureG1"));

    // --- Additional: Verify the signature is valid ---
    let pk = PublicKeyG2::generate_using_secret_key(&sk, &params);
    let verified = signature.verify(&fr_messages, pk, params.clone()).is_ok();
    assert!(verified, "Signature should verify for issued messages and params");

    // Optionally, print for debug
    println!("Credential issued: {:?}", credential.messages);

    // --- Blockchain interaction (JS API) ---
    let client = reqwest::blocking::Client::new();
    let api_url = "http://localhost:3001";

    // Add a block for the issued credential
    let resp = client.post(&format!("{}/add", api_url))
        .json(&serde_json::json!({ "data": format!("CredentialIssued:{:?}", credential.messages) }))
        .send()
        .expect("Failed to add block");
    assert!(resp.status().is_success());

    // Get the chain
    let chain: serde_json::Value = client.get(&format!("{}/chain", api_url))
        .send()
        .expect("Failed to get chain")
        .json()
        .expect("Invalid JSON");
    println!("[JS Blockchain] Chain: {:#?}", chain);

    // Verify chain
    let valid: serde_json::Value = client.get(&format!("{}/verify", api_url))
        .send()
        .expect("Failed to verify chain")
        .json()
        .expect("Invalid JSON");
    println!("Chain valid? {}", valid["valid"]);
    assert!(valid["valid"].as_bool().unwrap_or(false));
}
