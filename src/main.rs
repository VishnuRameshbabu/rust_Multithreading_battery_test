//use std::os::windows::thread;  // import stmts
mod test;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender,Receiver};
use std::thread::{self, ThreadId};
fn main(){

static numbers: &'static [u32] = &[0,1,2,3,4,5,6,7,8,9];
//let mut childThreads = vec![];
// for i in numbers.iter(){
//     childThreads.push(thread::spawn(move || {println!("number is {} and it's thread id {:?} ", i, thread::current().id());}));   

// }

// for child in childThreads{ let test = child.join().unwrap();
// println!("{test:?}");}
//loop{
let mut ids:Vec<ThreadId> = Vec::with_capacity(numbers.len() as usize);
let (senderThread, receiverThread):(Sender<ThreadId>,Receiver<ThreadId>) = std::sync::mpsc::channel();
for i in numbers.iter(){
let senderSpace: Sender<ThreadId> = senderThread.clone();
 thread::spawn(move || {
        println!("{:?}",thread::current().id());
        senderSpace.send(thread::current().id()).unwrap();
});
ids.push(receiverThread.recv().unwrap())
}
// for i in numbers.iter(){
//     ids.push(receiverThread.recv().unwrap())
// }
println!("Received messages queue:{:?}",ids);

}
