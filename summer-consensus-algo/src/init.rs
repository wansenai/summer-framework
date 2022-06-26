use crate::state::node;
use std::sync::{Arc,Mutex};
use std::thread;
fn main(){
    loop{
        let node = Arc::new(Mutex::new(Follower::new(0usize)));
    
    }
}