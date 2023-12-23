mod complexe;
use complexe::Complexe;

struct JuliaDescriptor {
    c:Complexe,
    divergence_threshold_square: f64,
    
}
impl JuliaDescriptor {
    
    
    fn fonction_calcul(&self, mut a: Complexe) -> Complexe {
        let mut i = 0;
        let maxi = 100;
    
        while i < maxi {
            a = complexe::addition(complexe::multiplication(a, a), self.c);
            println!("{}+i{}",a.re,a.im);
            if a.re*a.re+a.im*a.im>self.divergence_threshold_square {
                break;
            }
           
            i += 1;
        }
    a
    }
}


fn main(){
    let jd = JuliaDescriptor{c : Complexe{re:0., im:1.}, divergence_threshold_square:237678676.};
    let c= jd.fonction_calcul(Complexe{re:3.,im:3.});
    println!("{}+ {}i",c.re,c.im);
/*
    let c1 = Complexe{re:2.0,im:3.0};
    let c2=Complexe{re:5.0,im:6.0};
    let c=complexe::addition(c1,c2);
    println!("{}+ {}i",c.re,c.im);

*/}