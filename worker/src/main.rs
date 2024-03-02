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
    println!("serialized_size: {}\n", serialized_size);

    let data_size = data.len() as u32;
    println!("data_size: {}\n", data_size);

    let total_size = serialized_size + data_size ;
    println!("\ntotal_size :  {}", total_size);

    // Envoi de la taille totale du message
    stream.write_all(&total_size.to_be_bytes()).expect("failed to send total size");
    println!( "\nStep 1 passed");

    // Envoi de la taille du message JSON
    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send total size JSON");
    println!( "\nStep 2 passed");


    // Envoi du message JSON
    stream.write_all(&serialized.as_bytes()).expect("failed to send serialized message");
    println!( "\nStep 3 passed");

    // Envoi des données binaires
    if !data.is_empty() {
        for byte in data {
            stream.write_all(&[*byte]).expect("failed to send data");
        }
        println!("\nStep 4 passed");
    }

    reception_message(stream);
}




pub fn reception_message(mut stream: &TcpStream) {
    println!("Étape 1 : Début de la réception du message");
    let mut size_bytes = [0u8; 4];
    stream.read_exact(&mut size_bytes);
    println!("Étape 2 : Lecture de size_bytes terminée, valeur : {:?}", size_bytes);

    let mut new_size_json = [0; 4];
    stream.read_exact(&mut new_size_json);
    println!("Étape 3 : Lecture de new_size_json terminée, valeur : {:?}", new_size_json);

    let msg_size = u32::from_be_bytes(new_size_json) as usize;
    println!("Étape 4 : Conversion de new_size_json en usize terminée, valeur : {}", msg_size);

    let mut json_buffer = vec![0u8; msg_size as usize];
    stream.read_exact(&mut json_buffer);
    println!("Étape 5 : Lecture du buffer JSON terminée, valeur : {:?}", json_buffer);

    let img_data: &mut Vec<u8> = &mut Vec::new();  
    println!("Étape 6 : Initialisation de img_data terminée, valeur : {:?}", img_data);

    let mut data_binaire_taille = 0;
    if msg_size >= size_bytes.len() {
        data_binaire_taille = msg_size - size_bytes.len();
        println!("Étape 7 : Calcul de data_binaire_taille terminé, valeur : {}", data_binaire_taille);
        println!("msg_size : {}", msg_size);
        println!("size_bytes.len() : {}", size_bytes.len());
    } else {
        eprintln!("Erreur Étape 7 : msg_size est plus petit que size_bytes.len()");
        println!("msg_size : {}", msg_size);
        println!("size_bytes.len() : {}", size_bytes.len());
    }    

    if data_binaire_taille > 0 {
        *img_data = vec![0; data_binaire_taille];
        stream.read_exact(img_data);
        println!("Étape 8 : Lecture des données binaires terminée, valeur : {:?}", img_data);
    }

    let message = String::from_utf8_lossy(&json_buffer).into_owned();
    println!("Étape 9 : Conversion du buffer JSON en chaîne de caractères terminée, valeur : {}", message);

    // Traitement du message
    match parse_json_string(&message) {
        Ok(parsed_message) => {
            match parsed_message {
                Message::FragmentTask(fragment_task) => {
                    // Si le message est de type FragmentTask, calculer le résultat
                    let Ok((fragment_result, data)) = calculate_fragment(&fragment_task) else { todo!() };
                    
                    println!("\nMessage reçu : \n{:?}", fragment_result);
                    //println!("data : {:?}", data);

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
