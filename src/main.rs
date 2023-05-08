use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader, SeekFrom};


fn count_lines(arquivo: &mut File) -> i32 {
    let mut count = 0;
    let reader = BufReader::new(arquivo);
    for _line in reader.lines() {
        count += 1;
    }
    count
}


fn main() -> std::io::Result<()> {
    let path = "writethis.txt";
    let mut f = OpenOptions::new()
        .append(true)
        .read(true)
        .open(path)
        .expect(&format!("Couldn't open file {}", path));

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("{}", contents);

    f.write_all(b"\nPra esquecer dos meus anseios e viver em paz\n")
        .expect("Couldn't write file");

    // Move the file cursor to the beginning of the file
    f.seek(SeekFrom::Start(0)).expect("Couldn't seek to the beginning of the file");

    let counter = count_lines(&mut f);
    println!("Line Count {}", &counter);

    Ok(())
}
