use messages::messages::{FragmentResult,FragmentRequest,FragmentTask, Message, JuliaDescriptor};
use messages::messages::calculate_fragment;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::str;

pub fn send_message(mut stream: &TcpStream, message: Message, data: &Vec<u8>) {

    println!("\nstream : {:?}\n", stream);
    println!("\nmessage : {:?}\n", message);

    let serialized = serde_json::to_string(&message).expect("failed to serialize object");
    println!("\nserialized:  {}", serialized);

    let serialized_size = serialized.len() as u32;
    println!("\nserialized_size :  {}", serialized_size);

    let size_bytes = serialized_size.to_be_bytes(); 
    println!("\nsize_bytes : {:?}", size_bytes);
    
      // Envoi de la taille totale du message
      stream.write_all(&size_bytes);

      // Envoi de la taille du message JSON
      stream.write_all(&size_bytes);
  
      // Envoi du message JSON
      stream.write_all(serialized.as_bytes());

      println!("\ndata.len: {}\n", data.len());
      if data.len() > 0 {
        for byte in data {
          stream.write_all(&[*byte]).expect( "Error writing to stream" );
        }
      }

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
    // Récupération du message
    let mut size_bytes = [0u8; 4];
    stream.read_exact(&mut size_bytes).expect("Erreur lors de la lecture de la taille du message");
    
    let mut new_size_json = [0; 4];
    let mut size_json = u32::from_be_bytes(new_size_json) as usize;
    stream.read_exact(&mut new_size_json).expect("Erreur lors de la lecture de la taille du message");
    
    let msg_size = u32::from_be_bytes(new_size_json) as usize;

    println!("\n msg_size  : {:?}\n", msg_size);

    let mut json_buffer = vec![0u8; msg_size as usize];
    stream.read_exact(&mut json_buffer).expect("Erreur lors de la lecture du message JSON");

    let img_data: &mut Vec<u8> = &mut Vec::new();  

    let data_binaire_taille= msg_size - size_json;
        if data_binaire_taille> 0 {
                *img_data = vec![0; data_binaire_taille];
                stream.read_exact(img_data);
        }

    // Affichage du message dans la console en tant que chaîne de caractères
    let message = String::from_utf8_lossy(&json_buffer).into_owned();
    println!("{} ça marche", message);


    // Traitement du message
    match parse_json_string(&message) {
        Ok(parsed_message) => {
            match parsed_message {
                Message::FragmentTask(fragment_task) => {
                    println!("oui ?");
                    // Si le message est de type FragmentTask, calculer le résultat
                    let Ok((fragment_result, data)) = calculate_fragment(&fragment_task) else { todo!() };
                    
                    for i in 0..data.len() {
                        img_data.extend(data[i].zn.to_be_bytes());
                        img_data.extend(data[i].count.to_be_bytes());
                    }
                    println!("\n fragment_result: {:?} \n", &fragment_result);
                    // Envoi du FragmentResult
                    send_message(&stream, Message::FragmentResult(fragment_result), img_data);
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

    println!("FragmentRequest = : {:#?}\n", fragment_request);
    let stream = TcpStream::connect(server_address).unwrap();
    let img_data: &Vec<u8> = &mut &Vec::new();
    let message_send: Message = Message::FragmentRequest(fragment_request);
    let a = send_message(&stream,message_send, img_data);
    println!("{:?}",a);
  
}
