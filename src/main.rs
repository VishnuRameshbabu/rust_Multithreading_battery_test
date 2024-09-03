    //use std::os::windows::thread;  // import stmts
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{Sender,Receiver};
    use std::thread::{self, Thread, ThreadId};
    use std::collections::HashMap;


    #[derive(Debug,Clone)]
    pub struct batteryInformation{
            threadId: ThreadId,
            command:String,
    }   // a mock struct for sending data

    
    fn main(){
    let mut threadMap:HashMap<ThreadId,batteryInformation>=HashMap::new();
    static numbers: &'static [u32] = &[0,1,2,3,4,5,6,7,8,9];



    //loop{
    
    let mut ThreadIdArray = Arc::new(Mutex::new(Vec::<ThreadId>::new()));
    let (senderThread, receiverThread):(Sender<batteryInformation>,Receiver<batteryInformation>) = std::sync::mpsc::channel();
    for i in numbers.iter(){
    let senderSpace: Sender<batteryInformation> = senderThread.clone();
    let ThreadIdArray = Arc::clone(&ThreadIdArray);
   
   //As of now, trying to send data in unidirection and modifying it. 
    thread::spawn(move || {
            println!("{:?}",thread::current().id());
            let mut threadInfo = batteryInformation{threadId:thread::current().id(),command:String::from("test")};
            ThreadIdArray.lock().unwrap().push(threadInfo.threadId);
            senderSpace.send(threadInfo).unwrap();
           
            
    });
    let receivingBatteryInformation = receiverThread.recv().unwrap();
    threadMap.insert(receivingBatteryInformation.threadId,receivingBatteryInformation);
    }


    let ThreadIdArray = ThreadIdArray.lock().unwrap();
    println!("Received messages queue of Threads:{:?}",threadMap);
    println!("First thread id : {:?}",ThreadIdArray[0]);
    threadMap.get_mut(&ThreadIdArray[0]).unwrap().command = String::from("changed");
    println!("Received messages Id :{:?} , and command :{:?}",threadMap.get(&ThreadIdArray[0]).unwrap().threadId,threadMap.get(&ThreadIdArray[0]).unwrap().command);
    
    }

//  }