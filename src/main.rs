use notify_rust::{error::Error, Notification};
use std::{collections::{HashMap, LinkedList}, env};
fn main() -> Result<(), Error>{

    let mut args : LinkedList<String> = env::args().collect();

    let args_map = map_args(&mut args);
    let mut notification = Notification::new();
    notification.summary(args_map.get("summary").unwrap());
    notification.body(args_map.get("body").unwrap());
    notification.show()?;

    Ok(())
}

fn map_args(args : &mut LinkedList<String>) -> HashMap<String,String>{
    let mut hs : HashMap<String, String> = HashMap::new();
    while !args.is_empty(){
        let flag = args.pop_front().unwrap_or_default();
        match flag.as_str(){
            "-i" => {
                hs.insert("icon".to_string(), args.pop_front().expect("Error: no -i flag value"));
            },
            "-m" => {
                hs.insert("every * minutes".to_string(), args.pop_front().expect("Error: no -m flag value"));
            },
            "-s" => {
                hs.insert("summary".to_string(), args.pop_front().expect("Error: no -s flag value"));
            }
            "-b" =>{
                hs.insert("body".to_string(), args.pop_front().expect("Error: no -b flag value"));
            }
            _ => {}
        }
    }
    return hs;
}
