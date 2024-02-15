use serde_derive::{Deserialize, Serialize};
/*
*************************************************************************** */        

//use complexe::libs::Complexe;
//use complexe::libs::{addition,multiplication};
    //**********************JULIA DESCRIPTOR *************************************/

    
#[derive(Clone, Copy, Debug,Serialize,Deserialize)] 
pub struct JuliaDescriptor {
    c: Complexe,
    divergence_threshold_square: f64,
}

impl JuliaDescriptor {
    pub fn fonction_calcul(&self, mut a: Complexe, max_iteration : u16) -> Complexe {
        let mut i = 0 ;
        
        let maxi = 100;

        while i < max_iteration {
            a = addition(multiplication(a, a), self.c);
            //println!("{}+i{}", a.re, a.im);
            if a.re * a.re + a.im * a.im > self.divergence_threshold_square {
                break;
            }

            i += 1;
        }

        a
    }
}

//************************* Struct**************************************************** */

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Id {
    offset: u8,
    count: u8,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Fractal {
    Julia(JuliaDescriptor),
}


#[derive(Debug, Serialize, Deserialize,Clone)]
struct Resolution {
    nx: u16,
    ny: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Range {
    min: Point,
    max: Point,
}

#[derive(Debug, Serialize, Deserialize)]
struct U8Data {
    offset: u32,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PixelData {
    offset: u32,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PixelIntensity {
    zn: f32,
    count: f32,
}
//************************************FRAGMENT TASK***************************** */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct FragmentTask {
    id: Id,
    fractal: Fractal,
    max_iteration: u16,
    resolution: Resolution,
    range: Range
}

//************************ FragmentRequest********************************************************/
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct FragmentRequest{
   pub worker_name:String,
   pub maximal_work_load: u32
}

//****************************FragmentResult************************************************** */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct FragmentResult{
 id: Id,
 resolution: Resolution,
 range:Range,
 pixels:PixelData

}
//****************DIFFERENT MESSAGE***************************** */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Message {
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
    FragmentRequest(FragmentRequest),
}

//ù*************** functions ****************************************
pub fn parse_json_string(json_string: &str) -> Result<Message, serde_json::Error> {
    let parsed_data: Message = serde_json::from_str(json_string)?;
    Ok(parsed_data)
}
//***********************COMPLEXEE */

// ********************************** Nombre complexe *****************************************
#[derive(Clone, Copy, Debug, Serialize, Deserialize)] 
pub struct Complexe {
    pub re: f64,
    pub im: f64,
}

pub fn addition(c1: Complexe, c2: Complexe) -> Complexe {
        Complexe {
            re: c1.re + c2.re,
            im: c1.im + c2.im,
        }
    }

pub fn multiplication(c1: Complexe, c2: Complexe) -> Complexe {
        Complexe {
            re: c1.re * c2.re - c1.im * c2.im,
            im: c1.re * c2.im + c1.im * c2.re,
        }
    }


//µµµµµ*************************************** FONCTION***********
pub fn calculate_fragment(fragment_task: &FragmentTask) -> FragmentResult {
    // Récupérer les informations nécessaires du FragmentTask
    let resolution = &fragment_task.resolution;
    let range = &fragment_task.range;

    // Calculer les valeurs des pixels pour la fenêtre spécifiée
    let mut pixels_data = PixelData {
        offset: 0,
        count: 0,
    };

    // Vecteur pour stocker les valeurs des pixels
    let mut pixels: Vec<PixelIntensity> = Vec::new();

    for x in 0..resolution.nx {
        println!("{}", x);
        for y in 0..resolution.ny {
            // Convertir les coordonnées pixel en coordonnées physiques dans la fenêtre
            let phys_x = range.min.x + (x as f64 / resolution.nx as f64) * (range.max.x - range.min.x);
            let phys_y = range.min.y + (y as f64 / resolution.ny as f64) * (range.max.y - range.min.y);

            // Créer un complexe à partir des coordonnées physiques
            let complexe = Complexe { re: phys_x, im: phys_y };

            // Extraire le JuliaDescriptor de l'enum Fractal
            if let Fractal::Julia(julia_desc) = &fragment_task.fractal {
                // Utiliser la fonction de calcul pour obtenir la valeur du pixel
                let result = julia_desc.fonction_calcul(complexe,fragment_task.max_iteration);

                // Ajouter la valeur du pixel au vecteur 'pixels'

                
                pixels.push(PixelIntensity { zn: result.re, count: result.im });
            }
        }
    }

    // Remplir les données du pixel dans pixels_data (à remplacer par vos calculs réels)
    pixels_data.offset = 0;
    pixels_data.count = pixels.len() as u32;

    // Créer le FragmentResult avec les résultats des calculs
    let fragment_result = FragmentResult {
        id: fragment_task.id.clone(),
        resolution: resolution.clone(),
        range: range.clone(),
        pixels: pixels_data,
    };

    fragment_result
}
