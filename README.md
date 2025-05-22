# Solana email proofs

Verify email authenticity with zero-knowledge proofs and retrieve the result in Solana

> ⚠️ **In development**  — to see latest updates and information on how to run *Solana email proofs*, check out the `anchor` branch.

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

## Disclaimer
Use it at your own risk. This repository is experimental and unaudited. Do not use in production. 

## License

This project is licensed under the Apache2 license. See [LICENSE](https://github.com/risc0-labs/r0-zkEmail/blob/main/LICENSE).

