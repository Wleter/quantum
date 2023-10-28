use std::{fs::File, io::Write};

pub fn save_vec(filename: &str, data: Vec<Vec<f64>>, header: Vec<&str>) -> Result<(), std::io::Error> {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();

    let mut buf = header.join("\t");
    for value in data {
        let line= value.iter()
            .map(|x| format!("{:e}", x))
            .collect::<Vec<String>>()
            .join("\t");

        buf.push_str(&format!("\n{line}"));
    }

    let mut file = File::create(format!("{path}/data/{filename}.dat"))?;
    file.write_all(buf.as_bytes())?;

    Ok(())
}

pub fn save_parameter_change(filename: &str, parameter: Vec<f64>, data: Vec<Vec<f64>>, header: Vec<&str>) -> Result<(), std::io::Error> {
    let combined_data = parameter.iter()
        .zip(data.iter())
        .map(|(p, d)| [vec![*p], d.to_owned()].concat())
        .collect();

    save_vec(filename, combined_data, header)
}