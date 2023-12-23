mod complexe;
mod Juliadescriptor;
mod Point;

use complexe::Complexe;
use Juliadescriptor::JuliaDescriptor;
use Point::Point;


struct FragmentTask {
    id: Id,
    fractal: Fractal,
    max_iteration: u16,
    resolution: Resolution,
    range: Range
}

struct Id {
    offset: u8,
    count: u8
}

struct Fractal {
    Julia: JuliaDescriptor,
}

struct Resolution {
    nx: u16,
    ny: u16,
}

struct Range {
    min: Point,
    max: Point,
}