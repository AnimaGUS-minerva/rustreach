extern crate base64;
extern crate cose;
extern crate pem;
extern crate x509_parser;
extern crate ring;

use x509_parser::pem::pem_to_der;
use x509_parser::parse_x509_der;
//use ring::signature::{self, KeyPair};

//pub struct Voucher {
//    raw_voucher: Vec<u8>,
//}

static MASA_PEM: &'static [u8] = include_bytes!("../data/00-D0-E5-02-00-2D/masa.crt");

#[cfg(test)]
mod tests {
    //  use std::io::prelude::*;
    use std::fs::File;
    use std::io::Read;
    // use std::u8;
    // use self::base64::{decode};
    // use base64;
    use cose;

    //fn voucher_load(buffer: Vec<u8>) -> voucher {
    //}

    #[test]
    fn it_loads_cbor_1() {
        /* open the file */
        let mut cborin = File::open("data/00-D0-E5-F2-00-02/voucher_00-D0-E5-F2-00-02.vch").unwrap();

        let mut buffer = Vec::new();

        // read the whole file
        cborin.read_to_end(&mut buffer).unwrap();

        //println!("buffer1 => {} {:x?}", buffer.len(), buffer);

        //println!("buffer2 => {} {:x?}", buffer.len(), buffer);

        // if it was base64 encoded...
        //let voucher_bytes = &base64::decode(&mut buffer).unwrap();

        let voucher_bytes = buffer;
        //println!("voucher_bytes => {:x?}", voucher_bytes);

        let result = cose::decoder::decode_signed_object(&voucher_bytes, None);
        assert!(result.is_err());

        let mut coseobjs = result.unwrap();
        assert_eq!(coseobjs.len(), 1); // Vec of length 1.
        let firstsig = coseobjs.pop().unwrap();

        assert_eq!(firstsig.signature_type, cose::SignatureAlgorithm::ES256);

        /* get the signing certificate */
        let res = x509_parser::pem::pem_to_der(::MASA_PEM);
        match res {
            Ok((rem, pem)) => {
                assert!(rem.is_empty());

                assert_eq!(pem.label, String::from("CERTIFICATE"));
                let res_x509 = x509_parser::parse_x509_der(&pem.contents);
                assert!(res_x509.is_ok());
            },
            _ => panic!("PEM parsing failed: {:?}", res),
        }
        //let testcert = EndEntityCert::from();

        //Ok(CoseSignature {
        //signature_type: signature_algorithm,
        //signature: signature_bytes,
        //signer_cert: ee_cert,
        //certs: certs,
        //to_verify: sig_structure_bytes,
        //})

    }
}

