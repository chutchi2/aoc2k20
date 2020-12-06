pub mod lib {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::iter::Iterator;
    use std::path::Path;
    pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }

    pub fn record_parser(lines: Vec<String>) -> Vec<i64> {
        let mut records = Vec::new();
        for line in lines {
            records.push(line.parse::<i64>().unwrap());
        }
        records
    }

    pub fn job_parser(records: &Vec<char>) -> Vec<char> {
        let mut job_set: Vec<char> = Vec::new();
        for slice in records.chunks(3) {
            job_set.push(slice[0]);
            println!("{:?}", slice);
        }
        job_set
    }
}
