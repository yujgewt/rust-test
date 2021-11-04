use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    println!("Hello, world!");

    let s = "Abc";
    
    println!("test string : {}",s);


    let hex_list = s.as_bytes();
    println!("{:?}",hex_list);
    print_type_of(&hex_list);
    let hex_list2:Vec<u8> = hex_list.iter().map(|x| x-65).collect();
    print_type_of(&hex_list2);
    println!("{:?}",hex_list2);

    for i in hex_list2{
        println!("0x{:X}",i);
    }


    /*
    let a:Aes = Aes{
        key:32,
        plaintext:32
    };
    
    println!("{:?}",a);
    */
}



#[derive(Debug)]
struct Aes {
    key:u32,
    plaintext:u32
}





impl Aes {
    fn encrypt(&self) {

    }
    fn decrypt(&self) {
        
    }

}


