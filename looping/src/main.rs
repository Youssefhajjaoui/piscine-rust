use std::io;

fn main(){
    let mut conter  = 0;
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    loop{
        let mut input : String =String::from("");
        let res = io::stdin().read_line(&mut input);
        let resultat =  match res {
            Ok(val) => val,
            Err(err) => panic!("{}",err),
        };
        if input.trim() == "e"{
            println!("Number of trials: {}",conter);
            break;
        }else{
            conter+=1;
            println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        }
    }
}