use fvm::vm::{OpCode, Value, VM};
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16, FheUint8};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);
    // On the server side:
    set_server_key(server_keys);

    let a = 10u8;
    let b = 5u16;

    let enc_a = FheUint8::try_encrypt(a, &client_key)?;
    let enc_b = FheUint16::try_encrypt(b, &client_key)?;

    let mut vm = VM::new();
    let bytecode = [
        OpCode::Push(Value::Uint8(enc_a)),
        OpCode::Push(Value::Uint16(enc_b)),
        OpCode::Add,
    ];
    vm.execute(&bytecode);

    let encrypted_res = vm.pop();
    let clear_res: u16 = encrypted_res.as_int16().unwrap().decrypt(&client_key);
    assert_eq!(clear_res, 15);

    Ok(())
}
