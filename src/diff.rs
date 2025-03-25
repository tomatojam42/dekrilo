use std::path::PathBuf;

pub struct Diffargs {

}
impl Default for Diffargs {
    fn default() -> Self {
        Diffargs {

        }
    }
}

pub fn diff(file1: &PathBuf, file2: &PathBuf, d: Diffargs) -> Result<(),()> {
    let mut ranges: Vec<(usize,usize)> = vec![];
    let mut was_diff = (false, 0);
    let mut counter = 0;
    let file1 = match std::fs::read(file1){
        Ok(file) => file,
        Err(_e) => return Err(())
    };
    let file2 = match std::fs::read(file2){
        Ok(file) => file,
        Err(_e) => return Err(())
    };
    let size_diff = match file1.len() as isize - file2.len() as isize {
        0 => { 0 },
        n @ ..0 => { -n },
        n @ 1.. => { n }
    };
    counter = file1.iter().zip(file2.iter())
    .map(|(l,r)| {
        if l == r {
            if was_diff.0 {    
                was_diff.0 = false;
                ranges.push((was_diff.1, counter))
            }
        } else {
            if ! was_diff.0 {
                was_diff.1 = counter
            }
            was_diff.0 = true
        };
        counter += 1;
    }).count();
    if was_diff.0 {
        ranges.push((was_diff.1, counter))
    }
    ranges.iter()
    .map(|(a,b)| {
        println!("{a:x} .. {b:x}")
    })
    .count();
    if size_diff != 0 {
        println!("Также разница в размере: {size_diff:x}");
    }
    Ok(())
}