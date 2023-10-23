use std::{fs::File, io::Write};


pub fn save_vec(filename: &str, data: Vec<Vec<f64>>, header: Vec<&'static str>) -> Result<(), std::io::Error> {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();

    let mut buf = header.join("\t");
    for value in data {
        let line= value.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\t");

        buf.push_str(&format!("\n{line}"));
    }

    let mut file = File::create(format!("{path}/data/{filename}.dat"))?;
    file.write_all(buf.as_bytes())?;

    Ok(())
}