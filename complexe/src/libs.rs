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
    