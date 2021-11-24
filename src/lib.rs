
pub fn RotateLeft(a:u32,i:u8)-> u32 {
    a<<i | a>>(32-i)
}


pub fn SubBytes(mut a:u8)->u8{
    
    let sbox:[u8;256]  = [
                0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
                0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
                0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
                0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
                0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
                0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
                0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
                0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
                0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
                0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
                0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
                0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
                0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
                0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
                0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
                0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
            ];
            
    sbox[a as usize]
}


pub fn G(mut a:u32, round:u8)->u32{
    println!("input value : {:0X}",a);
    a = RotateLeft(a,8);
    println!("rotated value : {:0X}",a);
    

    let mut w:[u8;4] = [0;4];
    
    
    w[0] = ((a>>24) & 0xff) as u8;
    w[1] = ((a>>16) & 0xff) as u8;
    w[2] = ((a>>8) & 0xff) as u8;
    w[3] = (a & 0xff) as u8;
    

    let RC:[u8;10] = [
        0b1,
        0b10,
        0b100,
        0b1000,
        0b10000,
        0b100000,
        0b1000000,
        0b10000000,
        0b00011011,
        0b00110110,
    ];

    w[0] = SubBytes(w[0])^RC[round as usize];
    w[1] = SubBytes(w[1]);
    w[2] = SubBytes(w[2]);
    w[3] = SubBytes(w[3]);

    (w[3] as u32 )^((w[2] as u32)<<8)^((w[1] as u32)<<16)^((w[0] as u32)<<24)


}



pub fn KeyExpansion(key:[u32;4], round:u8)->[u32;4]{
    
    let mut w:[u32;4] = key;

    let mut result:[u32;4] = [0;4];

    result[0] = w[0]^G(w[3],round);
    result[1] = result[0]^w[1];
    result[2] = result[1]^w[2];
    result[3] = result[2]^w[3];


    result


}



pub fn MixColumns(text:[u8;16]){
    let m = [
                2,1,1,3,
                3,2,1,1,
                1,3,2,1,
                1,1,3,2
            ];
    
    
    
    for i in 0..4{
        let mut b:[u8;4] = [0;4];
        b[0] = m[i];
        b[1] = m[1*4+i];
        b[2] = m[2*4+i];
        b[3] = m[3*4+i];
        
        
        let mut s:u8 = 0;
        for j in 0..4{
            if b[j] == 2{
                
                s = s^(text[i*4+j]<<1);
            }else if b[j] ==3{
                
                s = s^(text[i*4+j]^(text[i*4+j]<<1));
            }else{
                s = s^text[i*4+j];
            }
        }
        println!("s = {:x}",s);
    }
}



pub fn ShiftRows(mut text:[u8;16])->[u8;16]{

    
    let mut temp:[u8;4]=[0;4];
    for i in 0..4{

        temp[0] = text[i+4*((0+i)%4)];
        temp[1] = text[i+4*((1+i)%4)];
        temp[2] = text[i+4*((2+i)%4)];
        temp[3] = text[i+4*((3+i)%4)];
        //println!("{}<-{}",i+4*(j%4),i+4*((j+i)%4));

        text[i+4*0] = temp[0];
        text[i+4*1] = temp[1];
        text[i+4*2] = temp[2];
        text[i+4*3] = temp[3];
    }

    text
}



fn AddRoundKey(key:[u8;16],text:[u8;16]) -> [u8;16]{
    xor(key,text)
}


fn xor(a:[u8;16],b:[u8;16])-> [u8;16] {
    //println!("{:?} {:?}",a,b);
    let mut n:[u8;16]=[0;16];
    for i in 0..16{
        n[i] = a[i]^b[i];
    }
    n

}








