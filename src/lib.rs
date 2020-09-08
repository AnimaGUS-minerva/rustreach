extern crate base64;
extern crate cose;
//use std::vec::Vec;


#[cfg(test)]
mod tests {
    //  use std::io::prelude::*;
    use std::fs::File;
    use std::io::Read;
    // use std::u8;
    // use self::base64::{decode};
    use base64;
    use cose;

    //fn voucher_load(buffer: Vec<u8>) -> voucher {
    //}

    #[test]
    fn it_loads_cbor_1() {
        /* open the file */
        let mut cborin = File::open("data/voucher_00-d0-e5-02-00-2d.b64").unwrap();
        let mut buffer = Vec::new();

        // read the whole file
        cborin.read_to_end(&mut buffer).unwrap();
        
        //println!("buffer1 => {} {:x?}", buffer.len(), buffer);
        // remove whitespace
        buffer.retain(|b| !b" \n\t\r\x0b\x0c".contains(b));
        
        //println!("buffer2 => {} {:x?}", buffer.len(), buffer);
        
        let voucher_bytes = &base64::decode(&mut buffer).unwrap();
        //println!("voucher_bytes => {:x?}", voucher_bytes);

        let result = cose::decoder::decode_signed_object(&voucher_bytes, None);
        assert!(result.is_err());

        let mut coseobjs = result.unwrap();
        assert_eq!(coseobjs.len(), 1); // Vec of length 1.
        let firstsig = coseobjs.pop().unwrap();

        assert_eq!(firstsig.signature_type, cose::SignatureAlgorithm::ES256);

        //Ok(CoseSignature {
        //signature_type: signature_algorithm,
        //signature: signature_bytes,
        //signer_cert: ee_cert,
        //certs: certs,
        //to_verify: sig_structure_bytes,
        //})
        
    }
}

