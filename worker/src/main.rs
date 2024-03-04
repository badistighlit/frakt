use messages::messages::{FragmentResult,FragmentRequest,FragmentTask, Message, JuliaDescriptor};
use messages::messages::calculate_fragment;
use std::{io::{self, Write, Read}, net::TcpStream};
use std::str;

mod newMain;

pub fn send_message(message: Message, data: &Vec<u8>) -> Result<(String, Vec<u8>), io::Error> {
    let mut stream = TcpStream::connect("localhost:8787").unwrap();

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

    stream.write_all(&total_size.to_be_bytes()).expect("failed to send total size");
    println!( "\nStep 1 passed");

    stream.write_all(&serialized_size.to_be_bytes()).expect("failed to send total size JSON");
    println!( "\nStep 2 passed");

    stream.write_all(&serialized.as_bytes()).expect("failed to send serialized message");
    println!( "\nStep 3 passed");

    stream.write_all(&data).expect("failed to send data");
    println!("{:?}", data);
    println!("\nStep 4 passed");
    
    
    let(messages,data) = match newMain::reception_message(&mut stream) {
        Ok(r) => r,
        Err(_e)=> panic!("An error occured during the reception of messages"),
    };
    println!("messages : {:?}", messages);
    println!("data : {:?}", data);
    Ok((messages, data))

}




pub fn reception_message(stream: &mut TcpStream) {
    
    println!("Étape 1 : Début de la réception du message");
    let mut size_bytes = [0u8; 4];
    let _ = stream.read_exact(&mut size_bytes);
    let total_size = u32::from_be_bytes(size_bytes);
    println!("Étape 2 : Lecture de size_bytes terminée, valeur : {:?}", total_size);

    let mut new_size_json = [0; 4];
    let _ = stream.read_exact(&mut new_size_json);
    println!("Étape 3 : Lecture de new_size_json terminée, valeur : {:?}", new_size_json);

    let msg_size = u32::from_be_bytes(new_size_json) as usize;
    println!("Étape 4 : Conversion de new_size_json en usize terminée, valeur : {}", msg_size);

    let mut json_buffer = vec![0u8; msg_size as usize];
    let _ = stream.read_exact(&mut json_buffer);
    println!("Étape 5 : Lecture du buffer JSON terminée, valeur : {:?}", json_buffer);

    let mut data_binaire_taille = 16;

    let mut img_data: Vec<u8> = vec![0_u8; data_binaire_taille];  
    println!("Étape 6 : Initialisation de img_data terminée, valeur : {:?}", img_data);

    if msg_size >= total_size as usize{
        data_binaire_taille = msg_size - total_size as usize;
        println!("Étape 7 : Calcul de data_binaire_taille terminé, valeur : {}", data_binaire_taille);
        println!("msg_size : {}", msg_size);
        println!("total_size.len() : {}", total_size as usize);
    } else {
        eprintln!("Erreur Étape 7 : msg_size est plus petit que size_bytes.len()");
        println!("msg_size : {}", msg_size);
        println!("total_size as usize : {}", total_size as usize);
    }    

    let _ = stream.read_exact(&mut img_data);
    println!("Étape 8 : Lecture des données binaires terminée, valeur : {:?}", img_data);
    

    let message = String::from_utf8_lossy(&json_buffer).into_owned();
    println!("Étape 9 : Conversion du buffer JSON en chaîne de caractères terminée, valeur : {}", message);

    match parse_json_string(&message) {
        Ok(parsed_message) => {
            match parsed_message {
                Message::FragmentTask(fragment_task) => {
                    let Ok((fragment_result, data)) = calculate_fragment(&fragment_task) else { todo!() };
                    
                    println!("\nMessage reçu : \n{:?}", fragment_result);

                    for i in 0..data.len() {
                        img_data.extend(data[i].zn.to_be_bytes());
                        img_data.extend(data[i].count.to_be_bytes());
                    }
                    println!("\n fragment_result: {:?} \n", &fragment_result);
                    let message_send: Message = Message::FragmentResult(fragment_result);

                    send_message(message_send, &mut img_data);
                }
                _ => {
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
    let fragment_request = FragmentRequest {
        worker_name: String::from("wsh"),
        maximal_work_load: 230,
    };

    println!("FragmentRequest = : {:#?}\n", fragment_request);
    let mut img_data: Vec<u8> = Vec::new();
    let message_send: Message = Message::FragmentRequest(fragment_request);
    let(messages,data) = match send_message(message_send, &img_data) {
        Ok(r) => r,
        Err(_e)=> panic!("truc"),
    };

    let messages_type = match parse_json_string(&messages) {
        Ok(r) => r,
        Err(_e)=> panic!("An error occured during the reception of messages"),
    };

    //voir le type du message
    let fragment_task = match messages_type {
        Message::FragmentTask(r) => r,
        _ => panic!("The received message is not a Fragment Task")
    };
    
    println!("fragment_task = {:?}", fragment_task);
    let Ok((fragment_result, pixels_data)) = calculate_fragment(&fragment_task) else { todo!() };

    for i in 0..data.len() {
        img_data.extend(pixels_data[i].zn.to_be_bytes());
        img_data.extend(pixels_data[i].count.to_be_bytes());
    }
    send_message(Message::FragmentResult(fragment_result), &img_data);
    
}
