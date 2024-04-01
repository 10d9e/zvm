#[cfg(test)]
mod tests {
    use crate::value::Value;
    use crate::vm::deserialize;
    use crate::vm::serialize;
    use crate::vm::OpCode;
    use crate::vm::VM;
    use tfhe::prelude::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder};
    use tfhe::{FheBool, FheUint128, FheUint16, FheUint32, FheUint64, FheUint8};

    #[test]
    fn test_add_i8_i16_promotion() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 20u8;
        let b = 5u16;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint16::try_encrypt(b, &client_key)?;

        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint16(enc_b)),
            OpCode::Add,
        ];
        vm.execute(&bytecode);

        let encrypted_res = vm.pop();
        let clear_res: u16 = encrypted_res.as_eint16().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 25);

        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 1u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Add,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 3);
        Ok(())
    }

    #[test]
    fn test_subtract() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 20u8;
        let b = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;

        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Sub,
        ];
        vm.execute(&bytecode);

        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 15);

        Ok(())
    }

    #[test]
    fn test_multiply() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Mul,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_divide() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 6u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Div,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 3);
        Ok(())
    }

    #[test]
    fn test_bitwise_and() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::And,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 1);
        Ok(())
    }

    #[test]
    fn test_bitwise_or() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Or,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 7);
        Ok(())
    }

    #[test]
    fn test_bitwise_xor() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Xor,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_bitwise_shift_left() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 1u8;
        let b = 1u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::ShiftLeft,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 2);
        Ok(())
    }

    #[test]
    fn test_bitwise_shift_right() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 1u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::ShiftRight,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 1);
        Ok(())
    }

    #[test]
    fn test_eq() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Eq,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_ebool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_neq() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Neq,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_ebool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_lt() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 1u8;
        let b = 2u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Lt,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_ebool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_gt() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 1u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Gt,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_ebool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_gte() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Gte,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_ebool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_lte() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Lte,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_ebool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;
        let b = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Min,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 4);
        Ok(())
    }

    #[test]
    fn test_max() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;
        let b = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Max,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_inc() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [OpCode::Push(Value::Euint8(enc_a)), OpCode::Inc];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_dec() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [OpCode::Push(Value::Euint8(enc_a)), OpCode::Dec];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 3);
        Ok(())
    }

    #[test]
    fn test_load() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Store(0),
            OpCode::Load(0),
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_store() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 6u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Store(0),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Store(1),
            OpCode::Load(0),
            OpCode::Load(1),
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_swap() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 6u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Swap,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_neg() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [OpCode::Push(Value::Euint8(enc_a)), OpCode::Neg];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, -5i8 as u8);
        Ok(())
    }

    #[test]
    fn test_mux() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = true;
        let b = 6u8;
        let c = 1u8;

        let enc_a = FheBool::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let enc_c = FheUint8::try_encrypt(c, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Ebool(enc_a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Push(Value::Euint8(enc_c)),
            OpCode::Mux,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_add_ciphertext_to_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 6u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::Add,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 11);
        Ok(())
    }

    #[test]
    fn test_subtract_ciphertext_from_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 6u8;
        let b = 4u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::Sub,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 2);
        Ok(())
    }

    #[test]
    fn test_multiply_ciphertext_with_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 6u8;
        let b = 4u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::Mul,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 24);
        Ok(())
    }

    #[test]
    fn test_divide_ciphertext_by_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 6u8;
        let b = 3u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::Div,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 2);
        Ok(())
    }

    #[test]
    fn test_and_ciphertext_with_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 3u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::And,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 1);
        Ok(())
    }

    #[test]
    fn test_and_plaintext_with_cipher_text() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 3u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::And,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 1);
        Ok(())
    }

    #[test]
    fn test_or_ciphertext_with_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 3u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::Or,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 7);
        Ok(())
    }

    #[test]
    fn test_or_plaintext_with_ciphertext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 3u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Or,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 7);
        Ok(())
    }

    #[test]
    fn test_xor_ciphertext_with_plaintext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 3u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Euint8(enc_a)),
            OpCode::Push(Value::Uint8(b)),
            OpCode::Xor,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_xor_plaintext_with_ciphertext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 3u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Xor,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_add_plaintext_to_ciphertext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 5u8;
        let b = 6u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Add,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 11);
        Ok(())
    }

    #[test]
    fn test_subtract_plaintext_from_ciphertext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 6u8;
        let b = 4u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Sub,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 2);
        Ok(())
    }

    #[test]
    fn test_multiply_plaintext_with_ciphertext() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 6u8;
        let b = 4u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Mul,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 24);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_divide_plaintext_by_ciphertext() {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 6u8;
        let b = 3u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key).unwrap();
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Div,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 2);
    }

    #[test]
    #[should_panic]
    fn test_min_ciphertext_with_plaintext() {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        let a = 4u8;
        let b = 5u8;
        let enc_b = FheUint8::try_encrypt(b, &client_key).unwrap();
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(a)),
            OpCode::Push(Value::Euint8(enc_b)),
            OpCode::Min,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_eint8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 4);
    }

    #[test]
    fn test_program_serialization_deserialization() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        // Define a sample program with a mix of opcodes, including a push with a value
        let original_program = [
            OpCode::Add,
            OpCode::Sub,
            OpCode::Mul,
            OpCode::Div,
            OpCode::And,
            OpCode::Or,
            OpCode::Xor,
            OpCode::ShiftRight,
            OpCode::ShiftLeft,
            OpCode::Eq,
            OpCode::Neq,
            OpCode::Lt,
            OpCode::Gt,
            OpCode::Gte,
            OpCode::Min,
            OpCode::Max,
            OpCode::Mux,
            //OpCode::Jmp(42),
            //OpCode::JmpIf(43),
            OpCode::Push(Value::Ebool(FheBool::try_encrypt(true, &client_key)?)),
            OpCode::Push(Value::Euint8(FheUint8::try_encrypt(8_u8, &client_key)?)),
            OpCode::Push(Value::Euint16(FheUint16::try_encrypt(16_u16, &client_key)?)),
            OpCode::Push(Value::Euint32(FheUint32::try_encrypt(32_u32, &client_key)?)),
            OpCode::Push(Value::Euint64(FheUint64::try_encrypt(64_u64, &client_key)?)),
            OpCode::Push(Value::Euint128(FheUint128::try_encrypt(
                128_u128,
                &client_key,
            )?)),
            OpCode::Push(Value::Bool(true)),
            OpCode::Push(Value::Uint8(8)),
            OpCode::Push(Value::Uint16(16)),
            OpCode::Push(Value::Uint32(32)),
            OpCode::Push(Value::Uint64(64)),
            OpCode::Push(Value::Uint128(128)),
            OpCode::Dup,
            OpCode::NoOp,
            OpCode::Inc,
            OpCode::Dec,
            OpCode::Load(77),
            OpCode::Store(88),
            OpCode::Swap,
        ];

        println!("Original program: {:?}", original_program.clone());

        // Serialize the program into bytes
        let serialized = serialize(&original_program);
        //println!("Serialized program: {:?}", hex::encode(serialized.clone()));

        // Deserialize the bytes back into opcodes
        let deserialized_program = deserialize(&serialized);

        // loop through deserialized program and assert each opcode matches original
        for (i, opcode) in deserialized_program.iter().enumerate() {
            assert_eq!(opcode.to_bytes(), original_program[i].to_bytes());
        }

        // Verify that the deserialized program matches the original
        //assert_eq!(deserialized_program, original_program);
        Ok(())
    }

    #[test]
    fn test_program_serialization_deserialization_types() -> Result<(), Box<dyn std::error::Error>>
    {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        // Define a sample program with a mix of opcodes, including a push with a value
        let original_program = [
            OpCode::Push(Value::Euint8(FheUint8::try_encrypt(8_u8, &client_key)?)),
            OpCode::Push(Value::Euint16(FheUint16::try_encrypt(16_u16, &client_key)?)),
            OpCode::Push(Value::Euint32(FheUint32::try_encrypt(32_u32, &client_key)?)),
            OpCode::Push(Value::Euint64(FheUint64::try_encrypt(64_u64, &client_key)?)),
            OpCode::Push(Value::Euint128(FheUint128::try_encrypt(
                128_u128,
                &client_key,
            )?)),
        ];

        // Serialize the program into bytes
        let serialized = serialize(&original_program);
        //println!("Serialized program: {:?}", hex::encode(serialized.clone()));

        // Deserialize the bytes back into opcodes
        let deserialized_program = deserialize(&serialized);

        // loop through deserialized program and assert each opcode matches original
        for (i, opcode) in deserialized_program.iter().enumerate() {
            assert_eq!(opcode.to_bytes(), original_program[i].to_bytes());
        }

        // Verify that the deserialized program matches the original
        //assert_eq!(deserialized_program, original_program);
        Ok(())
    }

    /*
    #[test]
    fn test_fibonacci() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        // Fibonacci program
        let bytecode = [
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(0, &client_key)?)), // n
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(1, &client_key)?)), // a
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(1, &client_key)?)), // b
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(10, &client_key)?)), // max
            OpCode::Store(0), // Store n
            OpCode::Store(1), // Store a
            OpCode::Store(2), // Store b
            OpCode::Store(3), // Store max
            OpCode::Load(0), // Load n
            OpCode::Load(3), // Load max
            OpCode::Gte, // n >= max
            OpCode::JmpIf(22), // Jump to end if n >= max
            OpCode::Load(1), // Load a
            OpCode::Load(2), // Load b
            OpCode::Add, // a + b
            OpCode::Store(4), // Store a + b
            OpCode::Load(2), // Load b
            OpCode::Store(1), // Store b
            OpCode::Load(4), // Load a + b
            OpCode::Store(2), // Store a + b
            OpCode::Load(0), // Load n
            OpCode::Inc, // n + 1
            OpCode::Store(0), // Store n + 1
            OpCode::Jmp(8), // Jump to beginning
            OpCode::Load(1), // Load a
        ];

        let mut vm = VM::new();
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 55);
        Ok(())
    }
    */
}
