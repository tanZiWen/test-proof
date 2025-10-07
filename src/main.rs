use std::ffi::{CString};
use std::os::raw::{c_char};
use std::slice;
use serde::Serialize;

unsafe extern "C" {
    fn Prove(input: *const c_char) -> *mut u8;
    fn FreeProof(ptr: *mut u8);
}

#[derive(Serialize, Debug)]
struct ProofInput {
    idx1: u32,
    idx2: u32,
    idx3: u32,
    sig1: String,
    sig2: String,
    sig3: String,
    cblk: String,
    blk: String,
}

fn generate_proof(input: &ProofInput) -> Result<Vec<u8>, String> {
    unsafe {
        let json = serde_json::to_string(input)
            .map_err(|e| format!("JSON serialization error: {}", e))?;
        
        let c_input = CString::new(json)
            .map_err(|e| format!("CString creation error: {}", e))?;

        let ptr = Prove(c_input.as_ptr());

        if ptr.is_null(){
            return Err("Proof generation failed".to_string());
        }

        let proof = slice::from_raw_parts(ptr, 736).to_vec(); 

        FreeProof(ptr);
        
        Ok(proof)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = ProofInput{
        idx1: 0,
        idx2: 1,
        idx3: 3,
        sig1: "ljE9Im1wDVEHpHsAqoaSp9sXBoOp9MlNIb009LlalmHYEyqE/40AEfTmMiwe3IsIEpJsWgOAAoOMpqtCDVpjj3GNeTuJyyOOrjCs/nMwzpKD02mzqUnOs8YBTICQ12bn".to_string(),
        sig2: "pQOz1Qzv4b2Jhqbz9w5sn9u9jcJyD0FojtKhSccld2UE5blRYy2x0O2wbb/ADVTlE8cpWHLDNowoix9Whr3bAfxqWV4wDb4M7wPgHoulV1Z4Hyp1SRHqFlw1jhs1BQ+L".to_string(),
        sig3: "oqAp50tTTpKyJ+4WO4dLy71KCN6iNLsOHDCDivSY1e/B87kaDm5G4uKoaXil936PEsEDw8opM3E2mx/Xo6CZq+oLeqvED90zKr2d8VXLvo6btc3zxfiJ+WQWetr5lPAQ".to_string(),
        cblk: "0100000003000000000000000100000000000000010000000000000067b4610973229d279f7fe83043242fcdb0bdfc8a2f09b7a982fcb4e1aaf34b83".to_string(),
        blk:  "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000dead00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
    };

    println!("Generating proof...");
    
    match generate_proof(&input) {
        Ok(proof) => {
            println!("Proof generated successfully!");
            println!("Proof length: {} bytes", proof.len());
            println!("Proof (hex): {}", hex::encode(&proof));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}