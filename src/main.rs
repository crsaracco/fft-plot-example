extern crate rustfft;
extern crate criterion_plot;
extern crate itertools_num;

mod sawtooth_oscillator;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use itertools_num::linspace;
use criterion_plot::prelude::*;
use std::path::Path;

const SAMPLES: usize = 44100;

fn main() {
    // Get a second's worth of data @ 44100 Hz sampling rate
    let mut saw_osc = sawtooth_oscillator::SawtoothOscillator::new();
    saw_osc.change_frequency(440.0);
    let mut input: Vec<Complex<f64>> = Vec::new();
    for _ in 0..SAMPLES {
        input.push(Complex::new(saw_osc.next_sample(SAMPLES as f64), 0.0));
    }

    for i in 0..SAMPLES {
        println!("input {}: {}", i, input[i]);
    }

    // Perform FFT on the samples
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); SAMPLES];
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(SAMPLES);
    fft.process(&mut input, &mut output);

    // The FFT output is mirrored after len/2, so just get half of the samples out.
    // Get the samples in polar form, and only care about the magnitude part.
    let mut graph_this: Vec<f64> = Vec::new();
    for i in 0..SAMPLES/2 {
        graph_this.push(output[i].to_polar().0);
    }

    for i in 0..SAMPLES/2 {
        println!("output {}: {}", i, graph_this[i]);
    }

    plot_vector(graph_this, "sawtooth", "magnitude.svg", true);
}


pub fn plot_vector(y_values: Vec<f64>, dataname: &'static str, filename: &'static str, log: bool) {
    let x_values = linspace::<f64>(0.0, y_values.len() as f64, y_values.len()).collect::<Vec<_>>();

    // Make a new Figure to plot our vector:
    let mut f = Figure::new();

    // Configure settings for the output of the plot:
    f.set(Font("Helvetica"));
    f.set(FontSize(16.0));
    f.set(Output(Path::new(filename)));
    f.set(Size(1336, 768));

    // If log, set y axis to log mode:
    if log {
        f.configure(Axis::BottomX, |a| a
            .set(Scale::Logarithmic)
            .set(Range::Limits(20.0, 44100.0/2.0))
        );
        f.configure(Axis::LeftY, |a| a
            .set(Scale::Logarithmic)
        );
    }

    // Configure the key for the plot
    f.configure(Key, |k| {
        k.set(Boxed::Yes)
            .set(Position::Inside(Vertical::Top, Horizontal::Left))
    });

    // Plot the vector (in memory):
    f.plot(
        Lines {
            x: x_values,
            y: y_values,
        },
        |l| {
            l.set(Color::Rgb(255, 0, 0))
                .set(Label(dataname))
                .set(LineType::Solid)
        }
    );

    // Spit out the plot to a .svg file:
    f.draw()
        .ok()
        .and_then(|gnuplot| {
            gnuplot.wait_with_output()
                .ok()
                .and_then(|p| String::from_utf8(p.stderr).ok())
        }).expect("ERROR occurred while plotting");
}