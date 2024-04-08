use tfhe::core_crypto::prelude::UnsignedNumeric;
use tfhe::integer::block_decomposition::DecomposableInto;
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ClientKey, ConfigBuilder, FheUint16, FheUint8};
use zvm::value::Value;
use zvm::vm::{OpCode, VM};

fn main2() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default()
        .enable_function_evaluation()
        .build();
    // Key generation
    let (client_key, server_keys) = generate_keys(config);
    // On the server side:
    set_server_key(server_keys);

    let a = 10u8;
    let b = 5u8;

    let enc_a = FheUint8::try_encrypt(a, &client_key)?;
    let enc_b = FheUint8::try_encrypt(b, &client_key)?;

    let enc_out = enc_a.bivariate_function(&enc_b, |a, b| a + b + 42);
    let enc_out2 = enc_a.bivariate_function(&enc_b, |a, b| a + b + 43);
    let enc_out3 = enc_a.bivariate_function(&enc_b, |a, b| a + b + 44);
    let enc_out4 = enc_a.bivariate_function(&enc_b, |a, b| a + b + 45);

    let out: u16 = enc_out.decrypt(&client_key);
    println!("out: {}", out);

    let out2: u16 = enc_out2.decrypt(&client_key);
    println!("out2: {}", out2);

    let out3: u16 = enc_out3.decrypt(&client_key);
    println!("out3: {}", out3);

    let out4: u16 = enc_out4.decrypt(&client_key);
    println!("out4: {}", out4);

    let enc_d = enc_a.map(|a| a * a);
    let d: u8 = enc_d.decrypt(&client_key);
    println!("d: {}", d);

    let mut vm = VM::new();
    let bytecode = [
        OpCode::Push(Value::Euint8(enc_a)),
        OpCode::Push(Value::Euint8(enc_b)),
        OpCode::Add,
    ];
    println!("bytecode: {:?}", bytecode);
    vm.execute(&bytecode);

    let encrypted_res = vm.pop();
    let clear_res: u16 = encrypted_res.as_eint16().decrypt(&client_key);
    assert_eq!(clear_res, 15);

    Ok(())
}

fn main44() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default()
        .enable_function_evaluation()
        .build();
    // Key generation
    let (client_key, server_keys) = generate_keys(config);
    // On the server side:
    set_server_key(server_keys);

    let a = 10u8;
    let b = 5u8;

    let enc_a: FheUint8 = try_encrypt_it(a, &client_key)?;

    //let enc_a = FheUint8::try_encrypt(a, &client_key)?;
    let enc_b = FheUint8::try_encrypt(b, &client_key)?;

    // execute a bivariate function
    let enc_out = enc_a.bivariate_function(&enc_b, |a, b| a + b);
    let out: u16 = enc_out.decrypt(&client_key);
    println!("out: {}", out);

    // execute a monovariate function
    let enc_out2 = enc_out.map(|a| a + 42);
    let out2: u16 = enc_out2.decrypt(&client_key);
    println!("out2: {}", out2);

    let enc_d = enc_out.map(|a| a * a);
    let d: u8 = enc_d.decrypt(&client_key);
    println!("d: {}", d);

    Ok(())
}

// create a proxy fn for try_encrypt
fn try_encrypt_it<T, Key, R>(value: T, key: &Key) -> Result<R, Box<dyn std::error::Error>>
where
    T: DecomposableInto<u64> + UnsignedNumeric,
    R: tfhe::prelude::FheTryEncrypt<T, Key>,
    <R as tfhe::prelude::FheTryEncrypt<T, Key>>::Error: 'static,
{
    Ok(R::try_encrypt(value, key)?)
}

use pest::Parser;
use zvm::lang::{parse_program, parse_program_pairs};
use zvm::lang::MyLanguageParser;
use zvm::lang::Rule;

fn main() {
    let source_code = std::fs::read_to_string("test.zed").unwrap();
    
    let parsed =
        MyLanguageParser::parse(Rule::program, &source_code).unwrap_or_else(|e| panic!("{}", e));
    let ast = parse_program_pairs(parsed); // Implement `parse_program` based on your AST and grammar

    println!("{:?}", ast);
    //let rust_code = compile_program(&ast); // Implement `compile_program`

    //std::fs::write("output.rs", rust_code).expect("Unable to write file");
}
