#![allow(dead_code)]

use std::fs;
use std::iter::Iterator;
type Float = f32;

fn smin<'a, T: PartialOrd>(x1: &'a T, x2: &'a T) -> &'a T {
    if *x1 < *x2 { x1 } else { x2 }
}

fn smax<'a, T: PartialOrd>(x1: &'a T, x2: &'a T) -> &'a T {
    if *x1 > *x2 { x1 } else { x2 }
}

fn min<'a, T: PartialOrd>(v: &'a [T]) -> &'a T {
    // v.iter().fold(&v[0], |ac, x| { if *ac < *x { ac } else { x } })
    v.iter().fold(&v[0], |ac, x| smin(ac, x))
}

fn max<'a, T: PartialOrd>(v: &'a [T]) -> &'a T {
    // v.iter().fold(&v[0], |ac, x| { if *ac > *x { ac } else { x } })
    v.iter().fold(&v[0], |ac, x| smax(ac, x) )
}

pub trait Numeric: PartialOrd+Copy+Into<f32>+Into<f64> {}

#[derive(Debug)]
pub struct Stream<T: Numeric> {
    data: Vec<T>,
    max: T,
    min: T,
}

impl<T: Numeric> Stream<T> {

    pub fn new<C, D>(collection: C) -> Self
        where C: Iterator<Item=D>,
              D: Into<T>
    {

        let data: Vec<T> = collection.into_iter().map(|d| d.into()).collect();
        let max = *max(&data);
        let min = *min(&data);

        Self {
            data: data,
            max: max,
            min: min,
        }

    }

    pub fn grow<C, D>(&mut self, new_data: C)
        where C: Iterator<Item=D>, D: Into<T>
        {
            let data: Vec<T> = new_data.into_iter().map(|d| d.into()).collect();
            let new_max = max(&data);
            let new_min = min(&data);

            if *new_max > self.max { self.max = *new_max }
            if *new_min < self.min { self.min = *new_min }
            self.data.extend(data)
        }

    pub fn frame(&self, ini: usize, end: usize) -> Frame<T> {
        let stream_slice = &self.data[ini..end];
        let min = *min(stream_slice);
        let max = *max(stream_slice);
        Frame {
            stream: stream_slice,
            ini: ini,
            end: end,
            min: min,
            max: max,
        }
    }

}

pub struct StreamBundle<T: Numeric> {
    streams: Vec<Stream<T>>,
    vscale: Float,
    hscale: Float,
}

// impl StreamBundle {
//     pub fn new(streams: Vec<Stream<T>>) -> StreamBundle {
//         StreamBundle {
//         }
//     }
// }


#[derive(Debug)]
pub struct Frame<'a, T: Numeric> {
    stream: &'a [T],
    ini: usize,
    end: usize,
    min: T,
    max: T,
}

impl<T: Numeric> Frame<'_, T> {

    pub fn optimal_scale(&self) -> f32 {
        (Into::<f32>::into(self.max) - Into::<f32>::into(self.min)) / 10_f32
    }

    pub fn raster(&self, vscale: f32, hscale: f32) -> Raster {

        let baseline = Into::<f32>::into(self.min) / vscale;

        let r: Vec<(usize,usize)> = self.stream.iter().enumerate()
            .map(|(i, j)| {
                let x = (i as f32 / hscale).floor() as usize;
                let y = (Into::<f32>::into(*j) / vscale - baseline).floor() as usize;
                (x, y)
            })
        .collect();

        let hmax = r[r.len()-1].0 +1;
        let vmax = (Into::<f32>::into(self.max) / vscale - baseline).floor() as usize +1;
        // let vmax = *max(&r.iter().map(|t| t.1).collect::<Vec<usize>>()[..]) +1;
        Raster {
            bins: r,
            hmax: hmax,
            vmax: vmax
        }
    }
}

pub struct Raster {
    bins: Vec<(usize, usize)>,
    hmax: usize,
    vmax: usize
}

impl Raster {

    pub fn raster(&self) -> Vec<Vec<u8>> {
        let mut raster: Vec<Vec<u8>> = Vec::new();
        for _h in 0..self.hmax {
            let mut line = Vec::new();
            for _v in 0..self.vmax {
                line.push(0u8)
            }
            raster.push(line)
        }
        for (i, j) in &self.bins {
            raster[*i][*j] = 1u8;
        }
        raster
    }
}


pub fn read_number_file(filename: &str) -> Vec<f32> {
    let r_file = fs::read_to_string(filename);
    let lines: Vec<f32> = r_file.unwrap().lines().map(|x| x.parse::<f32>().unwrap()).collect();
    lines
}


impl Numeric for f32 {}
// impl Numeric for f64 {}

// fn main() {
//     let data = read_number_file();
//     let s: Stream<f32> = Stream::new(data.iter().map(|x| *x));
//     let frame = s.frame(0, 100);
//     println!("{:?}", frame);
//     let optimal = frame.optimal_scale();
//     let r = frame.raster(optimal,5.);
//     for fila in r.raster() {
//         println!("{:?}", fila);
//     }
//     println!("{:?}", 5_f32 < 4.4_f32);
//     println!("{:?}", 5_f32 > 4_f32);
// }
