//Ceaser cipher with both Encription and decription
use std::io;
//Function that will shift the characters
fn shift(letter: char, key:i8)-> char{
    if key == 0 || key > 25 || key < -25 {
return letter;
    }
    else{
        let mut x=letter;
        //making the character Letter as integer ch.
        let ch= letter as i8;
        match key {
            //Positive Ke for encription 1 to 25
            key @ 0..=i8::MAX => {
                match ch {
                    //for capital letter
                    65..=90 => {
                        if ch.overflowing_add(key).1
                            || ch.overflowing_add(key).0 > 90
                        {
                            x = (ch + (key - 90 + 64)) as u8 as char;
                        } else {
                            x = (ch + key) as u8 as char;
                        }
                    }
                    //for small letters
                    97..=122 => {
                        if ch.overflowing_add(key).1
                            || ch.overflowing_add(key).0 > 122
                        {
                            x = (ch + (key - 122 + 96)) as u8 as char;
                        } else {
                            x = (ch + key) as u8 as char;
                        }
                    }
                    _ => { /* return the same character that it receieved */ }
                }
            }
            // Negative key for decription -25 to -1
            yek @ i8::MIN..=-1 => {
                let yek = -yek;
                match ch {
                     //for capital letter
                    65..=90 => {
                        if ch - yek < 65 {
                            let diff = 65 - (ch - yek);
                            x = ((90 - diff) + 1) as u8 as char;
                        } else {
                            x = (ch - yek) as u8 as char;
                        }
                    }
                    //for small letters
                    97..=122 => {
                        if ch - yek < 97 {
                            let diff = 97 - (ch- yek);
                            x = ((122 - diff) + 1) as u8 as char;
                        } else {
                            x = (ch - yek) as u8 as char;
                        }
                    }
                    _ => { /* return the same character that it receieved */ }
                }
            }
        }
        x
    }
    }
fn main(){
    //Encription of Decrition Determination
println!("You want Encription(Press 1) or Decription(Press 0)?");
let mut ch=String::new();
io::stdin().read_line(&mut ch).expect("falied to read");
let choice:i8=ch.trim().parse().expect("Error");
if choice==1{
    //Asking for text
println!("Enter the Text to to Encripted");
let mut texte=String::new();
io::stdin().read_line(&mut texte).expect("falied to read");
//Asking for key
println!("Enter the key (between 1 to 25)!");
let mut num=String::new();
io::stdin().read_line(&mut num).expect("falied to read");
let key:i8=num.trim().parse().expect("Error");
//Printing ciphertext
for c in texte.chars(){
print!("{}",shift(c, key));
}
}
else{
    //Asking for planetext
println!("Enter the Text to to Decripted");
let mut textd=String::new();
io::stdin().read_line(&mut textd).expect("falied to read");
//Asking for key
println!("Enter the negetive of the key (between -25 to-1)!");
let mut num=String::new();
io::stdin().read_line(&mut num).expect("falied to read");
let key:i8=num.trim().parse().expect("Error");
//Printing Planetext
for c in textd.chars(){
print!("{}",shift(c, key));
}
}
}