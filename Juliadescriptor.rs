mod complexe;
use complexe::Complexe;
struct JuliaDescriptor {
    c:Complexe,
    divergence_threshold_square: f64
}
fn main(){

    let c1 = Complexe{re:2.0,im:3.0};
    let c2=Complexe{re:5.0,im:6.0};
    let c=complexe::addition(c1,c2);
    println!("{}+ {}i",c.re,c.im);}