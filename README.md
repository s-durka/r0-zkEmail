# Solana email proofs

Verify email authenticity with zero-knowledge proofs and retrieve the result in Solana

> ⚠️ **In development**  — to see latest updates and information on how to run *Solana email proofs*, check out the `anchor` branch.

## Installation

In order to interact with the email proofs on Solana, you will need [Bonsol](https://bonsol.gitbook.io/docs/getting-started/installation) installed.

## Usage

To run the program, we need to download an email. In Google Mail, navigate to an email and use the hamburger menu to download the `.eml` file associated with the email. We then need to run our host, which will accept the domain and path to the email in order to verify the DKIM header. 

You can test the zk program *locally (without Bonsol)* by running:
```
cd zk_program/
RISC0_DEV_MODE=1 RUST_LOG=info cargo run --release -- <FROM_DOMAIN> <EMAIL_PATH>
```

#### Parameters

- `<FROM_DOMAIN>`: The domain the email comes from. For example, an email from Google Mail account will be `gmail.com`
- `<EMAIL_PATH>`: The path to the email `.eml` file.

### Example

```
RISC0_DEV_MODE=1 RUST_LOG=info cargo run --release -- gmail.com example.eml
```

### Bonsol
In order to run the program locally, you will need to set up Solana's local validator with Bonsol running — see how to [set up a local Bonsol environment](https://bonsol.gitbook.io/docs/developers/setup-a-local-environment).

Additionally, you need a *local input server* that will be hosting the `Email` struct encoded into bytes (see `zk_program/input-server`).

See the line: `generate_bonsol_inputs(&email_proof);` in `host/main.rs` as an example.
 
```bash
# in anchor_zkemail/
anchor deploy
# in zk_program/methods/guest/
bonsol build --zk-program-path .
bonsol deploy url --url http://localhost:8080 -m manifest.json
#...copy imageId from manifest.json execution_request.json...
bonsol execute -f execution_request.json --wait
```

## Disclaimer
Use it at your own risk. This repository is experimental and unaudited. Do not use in production. 

## License

This project is licensed under the Apache2 license. See [LICENSE](https://github.com/risc0-labs/r0-zkEmail/blob/main/LICENSE).
