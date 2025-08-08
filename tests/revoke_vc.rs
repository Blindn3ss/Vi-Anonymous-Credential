use ark_bls12_381::{Bls12_381, Fr};
use bbs_plus::prelude::{SignatureParamsG1, SecretKey, SignatureG1};
use ark_ff::UniformRand;
use sha2::Sha256;
use rand::thread_rng;
use BMGen::acumulator::Accumulator;
use BMGen::credential::Credential;
use ark_ec::AffineRepr;

#[test]
fn test_vc_revocation() {
    let mut rng = thread_rng();
    let mut acc = Accumulator::new();

    // Step 1: Accumulator secret key
    let sk_fr = Fr::rand(&mut rng);
    acc.acc_gen(sk_fr, 10);

    // Step 2: Issuer secret key (for signing)
    let seed = [0u8; 32];
    let sk = SecretKey::<Fr>::generate_using_seed::<Sha256>(&seed);

    // Step 3: Prepare parameters and message
    let message_count = 2;
    let params = SignatureParamsG1::<Bls12_381>::generate_using_rng(&mut rng, message_count);
    let fr_messages: Vec<Fr> = (0..message_count).map(|_| Fr::rand(&mut rng)).collect();

    // Step 4: Issuer creates a BBS+ signature over the holder's messages
    let signature = SignatureG1::new(&mut rng, &fr_messages, &sk, &params).unwrap();

    // Step 5: Issuer constructs the Credential object
    let credential = Credential {
        signature: signature.clone(),
        messages: fr_messages.clone(),
        issuer_pk: None,
    };

    // Save accumulator value before revocation
    let acc_val_before = acc.get_acc_val();

    // Generate witness to register x in issued_x
    let h = ark_bls12_381::G1Affine::new_unchecked(
        ark_bls12_381::g1::G1_GENERATOR_X,
        ark_bls12_381::g1::G1_GENERATOR_Y,
    ).into_group();
    let _witness = acc.gen_wit(&sk, credential.clone(), &h);

    // Revoke the credential
    let _delta = acc.del(&sk, credential);

    // Save accumulator value after revocation
    let acc_val_after = acc.get_acc_val();

    // The accumulator value should change after revocation
    assert_ne!(acc_val_before, acc_val_after, "Accumulator value should change after revocation");

    // Optionally, log issuance to the blockchain
    let client = reqwest::blocking::Client::new();
    let api_url = "http://localhost:3001";
    let resp = client.post(&format!("{}/add", api_url))
        .json(&serde_json::json!({ "data": format!("CredentialIssued:{:?}", signature) }))
        .send()
        .expect("Failed to add issuance block");
    assert!(resp.status().is_success());

    // Optionally, log revocation to the blockchain
    let client = reqwest::blocking::Client::new();
    let api_url = "http://localhost:3001";
    let resp = client.post(&format!("{}/add", api_url))
        .json(&serde_json::json!({ "data": format!("CredentialRevoked:{:?}", signature) }))
        .send()
        .expect("Failed to add revocation block");
    assert!(resp.status().is_success());

    println!("Accumulator value before revocation: {:?}", acc_val_before);
    println!("Accumulator value after revocation: {:?}", acc_val_after);
}
