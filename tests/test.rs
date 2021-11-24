use rusttest::*;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_KeyExpansion() {
        
        let mut key:[u32;4] = [0x2b7e1516,0x28aed2a6,0xabf71588,0x09cf4f3c];
        
        for i in 0..10{

            key  = rusttest::KeyExpansion(key,i);
            println!("{:0X}",key[0]);
            println!("{:0X}",key[1]);
            println!("{:0X}",key[2]);
            println!("{:0X}\n",key[3]);

        }
        
    }



    #[test]
    fn test_encryption(){

        //let mut key:[u32;4] = [0x2b7e1516,0x28aed2a6,0xabf71588,0x09cf4f3c];
        let mut key:[u8;16] =  [
                                0x2b,0x7e,0x15,0x16,
                                0x28,0xae,0xd2,0xa6,
                                0xab,0xf7,0x15,0x88,
                                0x09,0xcf,0x4f,0x3c
                                ];
        
        //SubBytes(mut a:u8)

        



        
    }


    #[test]
    fn test_ShiftRows(){

        //let mut key:[u32;4] = [0x2b7e1516,0x28aed2a6,0xabf71588,0x09cf4f3c];
        let mut key:[u8;16] =  [
                                0x2b,0x7e,0x15,0x16,
                                0x28,0xae,0xd2,0xa6,
                                0xab,0xf7,0x15,0x88,
                                0x09,0xcf,0x4f,0x3c
                                ];
        
        println!("{:?}",rusttest::ShiftRows(key));
        
    }


    #[test]
    fn test_Mixcolumns(){

        //let mut key:[u32;4] = [0x2b7e1516,0x28aed2a6,0xabf71588,0x09cf4f3c];
        let mut key:[u8;16] =  [
                                0x2b,0x7e,0x15,0x16,
                                0x28,0xae,0xd2,0xa6,
                                0xab,0xf7,0x15,0x88,
                                0x09,0xcf,0x4f,0x3c
                                ];
        
        println!("{:?}",rusttest::MixColumns(key));
        
    }



}

