use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BatteryInformation {
    threadId: thread::ThreadId,
    command: String,
    threadName: u32,
    sender: Sender<thread::ThreadId>,   // senderMainToThread, the main loop uses this to send information to the specific thread.
counter:u32}

fn main() {
    static NUMBERS: &'static [u32] = &[100,101, 102, 103, 104, 105, 106, 107, 108, 109,999];            // assuming these are batteries
    let threadIdMapCommon = Arc::new(Mutex::new(HashMap::<thread::ThreadId, BatteryInformation>::new()));       // a common Thread map to have the battery information for each battery thread.
    let (senderThreadToMain, receiverThreadToMain): (Sender<BatteryInformation>, Receiver<BatteryInformation>) = channel(); // a channel to send data from thread loop to main loop
    let mut threadHandles = vec![];
    for &num in NUMBERS.iter() {        // thread For loop
        let senderMainClone = senderThreadToMain.clone();       
        let threadIdMap = Arc::clone(&threadIdMapCommon);
        let (senderMainToThread, receiverMainToThread): (Sender<thread::ThreadId>, Receiver<thread::ThreadId>) = channel(); // creating an individual channel for each thread between main and to the specific thread.
        let handle = thread::spawn(move || {
            let threadId = thread::current().id();
            let mut counter = 0;
            loop {      // thread loop
                let mut batteryInformation:BatteryInformation;

                if let Ok(receivingThreadId) = receiverMainToThread.try_recv() {        // receives information from the main loop
                        let mut threadMap = threadIdMap.lock().unwrap();
                    println!("Thread Loop - Thread {:?} received battery Information Update from main loop: {:?}", threadId, threadMap.get(&receivingThreadId));
                }
                let mut counterString:String = counter.to_string();
                if counter == 5{
                        batteryInformation = BatteryInformation {
                           threadId,
                           command: String::from("Battery status COUNTER REACHED ") + &counterString,
                           threadName: num,
                           sender: senderMainToThread.clone(), 
                           counter: counter
                       };
               } else{
                       batteryInformation = BatteryInformation {
                               threadId,
                               command: String::from("Battery status OK ") + &counterString,
                               threadName: num,
                               sender: senderMainToThread.clone(), 
                               counter:counter
                           };
               }
                senderMainClone.send(batteryInformation.clone()).unwrap();
                {
                    let mut threadMap = threadIdMap.lock().unwrap();
                    threadMap.insert(threadId, batteryInformation.clone());
                }
                thread::sleep(Duration::from_secs(2));
                counter += 1;

            }
            
        });
        threadHandles.push(handle);
    }

    loop {

        if let Ok(receivingBatteryInformation) = receiverThreadToMain.recv() {          // this receives the informatio from the thread.
            println!("Main Loop received: {:?}", receivingBatteryInformation);
            let threadId = receivingBatteryInformation.threadId;
            let mut threadMap = threadIdMapCommon.lock().unwrap();
            if let Some(info) = threadMap.get_mut(&threadId) {
               
                if info.command == "Battery status COUNTER REACHED" {
                
                let mut message:String = String::from("COUNTER REACHED at 5 on Receiving End.. Modifying counter - ") + &(String::from(info.counter.to_string()));
                    info.command = message;
                } else {
                    info.command = String::from("Main Loop - Changing information counter - ") + &(String::from(info.counter.to_string()));
                }
            }

            println!("Main Loop -Updated battery  info for Thread ID to be sent {:?}: {:?}", threadId, threadMap.get(&threadId));

            receivingBatteryInformation.sender.send(threadId).unwrap();
        }
    }

}
