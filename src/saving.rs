use std::{fs::{create_dir_all, File}, io::Write, path::Path};

use num::complex::Complex64;


/// Saves data to a file in the curr_dir/data/subfolder/filename.dat
pub fn save_data(subfolder: &str, filename: &str, header: Vec<&str>, data: Vec<Vec<f64>>) -> Result<(), std::io::Error> {
    assert!(data.len() > 0, "Data is empty");
    let n = data[0].len();
    for d in &data {
        assert_eq!(d.len(), n, "Data length is not consistent");
    }

    let path = std::env::current_dir().unwrap();
    let path = format!("{}/data/{subfolder}", path.to_str().unwrap());

    let mut buf = header.join("\t");

    for i in 0..n {
        unsafe {
            let line = data
                .iter()
                .map(|x| format!("{:e}", x.get_unchecked(i)))
                .collect::<Vec<String>>()
                .join("\t");

            buf.push_str(&format!("\n{line}"));
        }
    }

    if !Path::new(&path).exists() {
        create_dir_all(&path)?;
    }

    let mut file = File::create(format!("{path}/{filename}.dat"))?;
    file.write_all(buf.as_bytes())?;

    println!("saved on {path}/{filename}.dat");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::saving::save_data;

    #[test]
    fn test_save_data() {
        let filename = "test";
        let header = vec!["x", "y"];
        let xs = vec![1.0, 2.0, 3.0];
        let ys = vec![4.0, 5.0, 6.0];

        let result = save_data("", filename, header, vec![xs, ys]);
        assert!(result.is_ok());
    }
}


#[deprecated(note = "Use save_data instead")]
#[allow(warnings)]
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

#[deprecated(note = "Use save_data instead")]
#[allow(warnings)]
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

#[deprecated(note = "Use save_data instead")]
#[allow(warnings)]
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
