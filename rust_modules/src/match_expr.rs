use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn req_status()->u32{
    200
}


pub fn match_ex(){
    let status =req_status();

    match status {
        200 => println!("success"),
        404 => println!("Not found"),
        _=> println!("error")
    }
}

struct Player{
    name:String,
    iq:u8,
    friends:u8
}

impl Player{
    fn with_name(name:&str)->Self{
        return Player{
             name:name.to_string(),
            iq: 0,
            friends: 0,
        }
    }

    fn get_friends(&self)->u8{
        return self.friends
    }

    fn set_friends(&mut self,count:u8){
        self.friends=count
    }
}

enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}


fn pay_by_credit(amt:u64){
    println!("processing credit payment of {}",amt)
}

fn pay_by_debit(amt:u64){
    println!("processing debit payment {}",amt)
}

fn paypal_redirect(amt:u64){
    print!("rendering to paypal for amount {}",amt);
}

impl PaymentMode{
    fn pay(&self,amount:u64){
        match self {
            PaymentMode::Credit=>pay_by_credit(amount),
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Paypal=>paypal_redirect(amount)
        }
    }
}

fn get_served_payment_mode()->PaymentMode{
    PaymentMode::Debit
}

pub fn pay_main(){
    let payment_mode =get_served_payment_mode();
    payment_mode.pay(512)
}

pub fn hash_ap(){
    let mut fruit = HashMap::new();
    fruit.insert("apple",10);
    fruit.insert("orange",12);
    fruit.insert("grapes",34);

    for(key,val) in &fruit{
        println!("i got {}{}",key,val)
    }

    let mut numbers: [u8;4]=[1,2,3,4];
    {
        let all:&[u8] =&numbers[..];
        println!("all of them :{:?}",all)
    }
    {
        let first_two:&mut [u8]=&mut numbers[0..2];
        first_two[0]=100;
        first_two[1]=200;
    }

    println!("look ma ! i can modify through slices:{:?}",numbers)
}

#[derive(Debug)]
struct WordCounter(HashMap<String,u64>);

impl WordCounter{
    fn new()->WordCounter{
        return WordCounter(HashMap::new());
    }

    fn increment(&mut self, word:&str){
        let key =word.to_string();
        let count =self.0.entry(key).or_insert(0);

        *count+=1;
    }

    fn displays(self){
        for(key,val) in self.0.iter(){
            println!("{}:{}",key,val)
        }
    }
}

pub fn exercise(){
    let arguments:Vec<String>=vec!["bimal.txt".to_string(),
                                   "samantha".to_string(),
                                   "weerawansa".to_string()];

    let filename = &arguments[0];
    println!("processing file {}",filename);
    let file = File::open(filename).expect("could not open the file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();


    for line in reader.lines(){
        let line = line.expect("could not read line");
        let words = line.split(" ");

        for word in words{
            if word ==""{
                continue
            }else{
                word_counter.increment(word);
            }
        }
    }

    word_counter.displays();


}