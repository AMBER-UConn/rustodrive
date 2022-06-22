use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, SendError};

use socketcan::CANSocket;

use crate::constants::ODriveCommand;


pub trait CANThreadCommunicator {
    /// This passes a message to the CANManager
    fn send_to_manager(&self, msg: ODriveMessage) {
        let can_send = self.to_manager();

        // take the message and send it over the channel
        match can_send.send(msg) {
            Ok(()) => {},
            Err(error) => panic!("Lost connection to CANManager thread: \n{}", error)
        }
    }

    /// This waits for a response from the CANManager and returns the result
    /// 
    /// This is responsible for returning any errors related to invalid commands
    /// and or any errors that occur during the odrive's command execution
    fn receive_from_manager(&self) -> ODriveResponse {
        // wait for the response from the thread and return it
        let can_recv = self.from_manager();

        match can_recv.recv() {
            Ok(response) => return response,
            Err(error) => panic!("Lost connection to CANManager thread: \n{}", error)
        }
    }

    /// This returns the send portion of the communication channel to the CANManager
    /// This thread ---> CANManager
    fn to_manager(&self) -> &Sender<ODriveMessage>;

    /// This returns the receive portion of the communication channel from the CANManager
    /// This thread <--- CANManager
    fn from_manager(&self) -> &Receiver<ODriveResponse>;
}
// TODO make an error type
pub struct CANManager {
    mpsc_receiver: Receiver<ODriveMessage>,
    thread_senders: Sender<ODriveResponse>,

    waiting: Vec<ODriveMessage>,
    socket: CANSocket,
}

impl CANManager {
    pub fn new(can_device: &str, receiver: Receiver<ODriveMessage>, senders: Sender<ODriveResponse>) -> Self {
        // Initialize CANSocket
        let socket = CANSocket::open(can_device).expect("Could not open CAN at can1");

        Self {
            mpsc_receiver: receiver,
            thread_senders: senders,
            waiting: vec![],
            socket
        }
    }

    fn send_to_CAN(&self) {
        
    }

    fn receive_from_CAN(&self) {

    }

    /// it attempts to match and messages that are waiting to one that was received
    fn match_messages() {

    }

    /// get the channel for a particular access to respond to
    fn get_axis_channel(&self, axis_id: usize) {
        
    }
}

pub struct ODriveMessage {
    axis_id: usize,
    command: ODriveCommand,
    data: [u8; 8]
}

impl ODriveMessage {
    fn can_id(&self) -> u16 {
        return (self.axis_id as u16) << 5 | (self.command.clone() as u16);
    }
}

pub enum ODriveError {
    FailedToSend,
}

pub enum ODriveResponse {
    Ok([u8; 8]),
    Err(ODriveError) 
}
