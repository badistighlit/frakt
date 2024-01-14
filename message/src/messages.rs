
/*
*************************************************************************** */        

//use complexe::libs::Complexe;
//use complexe::libs::{addition,multiplication};
    //**********************JULIA DESCRIPTOR *************************************/

    #[derive(Clone, Copy, Debug)] 
pub struct JuliaDescriptor {
    c: Complexe,
    divergence_threshold_square: f64,
}

impl JuliaDescriptor {
    pub fn fonction_calcul(&self, mut a: Complexe) -> Complexe {
        let mut i = 0 ;
        
        let maxi = 100;

        while i < maxi {
            a = addition(multiplication(a, a), self.c);
            println!("{}+i{}", a.re, a.im);
            if a.re * a.re + a.im * a.im > self.divergence_threshold_square {
                break;
            }

            i += 1;
        }

        a
    }
}

//************************* Struct**************************************************** */

#[derive(Debug)]
struct Id {
    offset: u8,
    count: u8,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum Fractal {
    Julia(JuliaDescriptor),
}

#[derive(Debug)]
struct Resolution {
    nx: u16,
    ny: u16,
}

#[derive(Debug)]
struct Range {
    min: Point,
    max: Point,
}

#[derive(Debug)]
struct U8Data {
    offset: u32,
    count: u32,
}

#[derive(Debug)]
struct PixelData {
    offset: u32,
    count: u32,
}

#[derive(Debug)]
struct PixelIntensity {
    zn: f32,
    count: f32,
}
//************************************FRAGMENT TASK***************************** */
struct FragmentTask {
    id: Id,
    fractal: Fractal,
    max_iteration: u16,
    resolution: Resolution,
    range: Range
}

//************************ FragmentRequest********************************************************/

struct FragmentRequest{
    worker_name:String,
    maximal_work_load:f32
}

//****************************FragmentResult************************************************** */
struct FragmentResult{
 id: Id,
 resolution: Resolution,
 range:Range,
 pixels:PixelData

}
//****************DIFFERENT MESSAGE***************************** */
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
    FragmentRequest(FragmentRequest),
}
//***********************COMPLEXEE */

// ********************************** Nombre complexe *****************************************
#[derive(Clone, Copy, Debug)] 
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



fn main(){



}     