use std::{fs::File, io::Write};

use num::complex::Complex64;

pub fn save_vec(
    filename: &str,
    data: Vec<Vec<f64>>,
    header: Vec<&str>,
) -> Result<(), std::io::Error> {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();

    let mut buf = header.join("\t");
    for value in data {
        let line = value
            .iter()
            .map(|x| format!("{:e}", x))
            .collect::<Vec<String>>()
            .join("\t");

        buf.push_str(&format!("\n{line}"));
    }

    let mut file = File::create(format!("{path}/data/{filename}.dat"))?;
    file.write_all(buf.as_bytes())?;

    Ok(())
}

pub fn save_param_change(
    filename: &str,
    parameter: Vec<f64>,
    data: Vec<Vec<f64>>,
    header: Vec<&str>,
) -> Result<(), std::io::Error> {
    let combined_data = parameter
        .iter()
        .zip(data.iter())
        .map(|(p, d)| [vec![*p], d.to_owned()].concat())
        .collect();

    save_vec(filename, combined_data, header)
}

pub fn save_param_change_complex(
    filename: &str,
    parameter: Vec<f64>,
    data: Vec<Complex64>,
    header: Vec<&str>,
) -> Result<(), std::io::Error> {
    let real_part = format!("{} re", header[1]);
    let im_part = format!("{} im", header[1]);
    let header = vec![header[0], &real_part, &im_part];

    let combined_data = parameter
        .iter()
        .zip(data.iter())
        .map(|(p, d)| vec![*p, d.re, d.im])
        .collect();

    save_vec(filename, combined_data, header)
}
