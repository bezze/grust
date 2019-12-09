mod draw;
mod interact;
mod core;
mod stream;

// use stream::Numeric;


fn test_raster() {
    let data = stream::read_number_file("/home/jason/Others/gits/grust/src/numbers");
    let s: stream::Stream<f32> = stream::Stream::new(data.iter().map(|x| *x));

    let frame = s.frame(0, 100);
    println!("{:?}", frame);
    let optimal = frame.optimal_scale();
    let r = frame.raster(optimal,5.);
    for fila in r.raster() {
        println!("{:?}", fila);
    }
}


fn main() {

    draw::start_ncurses_mode();
    {
        while core::start_interface().is_ok() { }
    };
    draw::end_ncurses_mode();

    // test_raster()
}
