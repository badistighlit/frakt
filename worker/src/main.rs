use messages::messages::{FragmentResult,FragmentRequest,FragmentTask, Message, JuliaDescriptor};
use messages::messages::calculate_fragment;
use std::io::{Write,Read};
use std::net::TcpStream;
use std::str;

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
    let result = stream
        .write_all(serialized.as_bytes())
        .expect("erreur envoie message");
    println!("{result:?} ");
    match stream.write_all(serialized.as_bytes()) {
        Ok(()) => println!("Message envoyé avec succès"),
        Err(err) => eprintln!("Erreur lors de l'envoi final : {}", err),
    }
    reception_message(stream);
}




pub fn reception_message(mut stream: &TcpStream) {
    // recuperation du msg
    let mut size_bytes = [0u8; 4];
    stream.read_exact(&mut size_bytes).expect("Erreur lors de la lecture de la taille du message");
    
    let mut size_json = [0; 4];
    stream.read_exact(&mut size_json).expect("Erreur lors de la lecture de la taille du message");
     
    
    let msg_size = u32::from_be_bytes(size_json);
    //println!("taille mssg size: {:?}", msg_size);

    let mut json_buffer = vec![0u8; msg_size as usize];
    //println!("json_buffer : {:?}",json_buffer);
    stream.read_exact(&mut json_buffer).expect("Erreur lors de la lecture du message JSON");
    //println!("json_buffer après réception : {:?}",json_buffer);
    let message = String::from_utf8_lossy(&json_buffer).into_owned();
    let fragment_request = parse_json_string(&message);


    
    match parse_json_string(&message) {
        Ok(parsed_message) => {
            match parsed_message {
                Message::FragmentTask(fragment_task) => {
                    // Si le message est de type FragmentTask, calculer le résultat
                    let fragment_result = calculate_fragment(&fragment_task);
                    
                    // Envoi du FragmentResult
                    send_message(&stream, Message::FragmentResult(fragment_result));
                }
                _ => {
                    // Traitez d'autres types de messages si nécessaire
                    dbg!(parsed_message);
                }
            }
        }
        Err(err) => {
            eprintln!("Erreur lors de la désérialisation du message JSON : {}", err);
        }
    }
}


fn parse_json_string(json_string: &str) -> Result<Message, serde_json::Error> {
    let parsed_data: Message = serde_json::from_str(json_string)?;
    Ok(parsed_data)
}








pub fn main() {
    let server_address = "localhost:8787";
    let fragment_request = FragmentRequest {
        worker_name: String::from("wsh"),
        maximal_work_load: 230,
    };

  
    let stream = TcpStream::connect(server_address).unwrap();

    let message_send: Message = Message::FragmentRequest(fragment_request);
    let a = send_message(&stream,message_send);
    println!("{:?}",a);
  
}
