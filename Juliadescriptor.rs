mod complexe;
use complexe::Complexe;
struct JuliaDescriptor {
    c:Complexe,
    divergence_threshold_square: f64,
    
}
impl JuliaDescriptor {
    fn fonction_calcul(&self, a: Complexe) -> Complexe {
        return complexe::addition(complexe::multiplication(a,a) , self.c);
    }
}


fn main(){
    let jd = JuliaDescriptor{c : Complexe{re:0., im:1.}, divergence_threshold_square:23.};
    let c= jd.fonction_calcul(Complexe{re:3.,im:3.});
    println!("{}+ {}i",c.re,c.im);
/*
    let c1 = Complexe{re:2.0,im:3.0};
    let c2=Complexe{re:5.0,im:6.0};
    let c=complexe::addition(c1,c2);
    println!("{}+ {}i",c.re,c.im);

*/}