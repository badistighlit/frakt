use messages::messages::{FragmentRequest, Message};
use std::io::{Write};
use std::net::TcpStream;

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    let serialized_size = serialized.len() as u32;
    let size_bytes = serialized_size.to_be_bytes(); 
    
      // Envoi de la taille totale du message
      stream.write_all(&size_bytes);

      // Envoi de la taille du message JSON
      stream.write_all(&size_bytes);
  
      // Envoi du message JSON
      stream.write_all(serialized.as_bytes());
  
      // Envoi des données binaires
      //stream.write_all(data);
    

    stream
        
        .write_all(&size_bytes)
        .expect("failed to send serialized size");
    let result = stream
        .write_all(serialized.as_bytes())
        .expect("failed to send message");
    println!("{result:?}");
}

pub fn main() {
    let server_address = "localhost:8787";
    let fragment_request = FragmentRequest {
        worker_name: String::from("zaaa"),
        maximal_work_load: 1000,
    };

  
    let stream = TcpStream::connect(server_address).unwrap();

    let message_send: Message = Message::FragmentRequest(fragment_request);
    send_message(&stream,message_send);
  
}
