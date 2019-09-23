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
    // use base64::{encode, decode};
    use base64;

    #[test]
    fn it_loads_cbor_1() {
        /* open the file */
        let mut cborin = File::open("data/voucher_00-d0-e5-02-00-2d.b64").unwrap();
        let mut buffer = Vec::new();

        // read the whole file
        cborin.read_to_end(&mut buffer).unwrap();
        
        println!("buffer1 => {} {:x?}", buffer.len(), buffer);
        // remove whitespace
        buffer.retain(|b| !b" \n\t\r\x0b\x0c".contains(b));
        
        println!("buffer2 => {} {:x?}", buffer.len(), buffer);
        
        let result = &base64::decode(&mut buffer);
        println!("buffer => {:x?}", buffer);
        match result {
            Ok(data) => {
                println!("read => {:x?}", data);
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}

