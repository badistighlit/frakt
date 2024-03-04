use std::{io::{self, Read}, net::TcpStream};


pub fn reception_message(stream: &mut TcpStream) -> Result<(String, Vec<u8>), io::Error> {
    
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


    let _ = stream.read_exact(&mut img_data);
    println!("Étape 8 : Lecture des données binaires terminée, valeur : {:?}", img_data);
    

    let message = String::from_utf8_lossy(&json_buffer).into_owned();
    println!("Étape 9 : Conversion du buffer JSON en chaîne de caractères terminée, valeur : {}", message);
    Ok((message,img_data))
}


