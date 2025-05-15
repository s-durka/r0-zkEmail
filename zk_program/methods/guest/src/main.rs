use cfdkim::{verify_email_with_key, DkimPublicKey};
use mailparse::parse_mail;
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
use slog::{o, Discard, Logger};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub from_domain: String,
    pub raw_email: Vec<u8>,
    pub public_key_type: String,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DKIMOutput {
    pub from_domain_hash: Vec<u8>,
    pub public_key_hash: Vec<u8>,
    pub verified: bool,
}


fn main() {
    let input: Vec<u8> = env::read_frame();
    let input: Email = postcard::from_bytes(&input).unwrap();

    println!("From domain: {:?}", &input.from_domain);
    println!("Pubkey type: {:?}", &input.public_key_type);
    println!("Pubkey: {:?}", &input.public_key);
    let raw_email_string = std::str::from_utf8(&input.raw_email).unwrap();
    println!("Raw email: {:?}", raw_email_string);

    let logger = Logger::root(Discard, o!());

    let parsed_email = parse_mail(&input.raw_email).unwrap();

    let public_key =
        DkimPublicKey::try_from_bytes(&input.public_key, &input.public_key_type).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(input.from_domain.as_bytes());
    let from_domain_hash = hasher.finalize().to_vec();

    let mut hasher = Sha256::new();
    hasher.update(&input.public_key);
    let public_key_hash = hasher.finalize().to_vec();

    let result =
        verify_email_with_key(&logger, &input.from_domain, &parsed_email, public_key).unwrap();

    let verified = match result {
        result if result.with_detail().starts_with("pass") => true,
        _ => false,
    };

    let output = DKIMOutput {
        from_domain_hash,
        public_key_hash,
        verified,
    };

    env::commit(&output);
}
