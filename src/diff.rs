use std::path::PathBuf;

use crate::DError;

/*
Интересный результат. Для файлов каждый по 256М:
В debug запуске:
    diff_loop_style: ~5.3 сек
    diff_func_style: ~3.7 сек
В release запуске:
    diff_loop_style: ~200 мс ~527900maxresident k
    diff_func_style: ~300 мс ~527900maxresident k

    Definition            loop_style   func_style
    User time (seconds):         0.13 0.23
    System time (seconds):       0.12 0.14
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.26 00.37
    Maximum resident set size (kbytes): 527920 527972
    Minor (reclaiming a frame) page faults: в обоих случаях сильно плавает
    (In)voluntary context switches: в обоих случаях сильно плавает
*/

pub struct DiffArgs<'a> {
    pub file1: &'a PathBuf,
    pub file2: &'a PathBuf,
    pub func: bool,
}

pub fn diff(a: DiffArgs) -> Result<(),DError> {
    let file1 = std::fs::read(a.file1).map_err(|_|DError::CantReadFile)?;
    let file2 = std::fs::read(a.file2).map_err(|_|DError::CantReadFile)?;
    let size_diff = match file1.len() as isize - file2.len() as isize {
        0 => { 0 },
        n @ ..0 => { -n },
        n @ 1.. => { n }
    };
    let ranges = if a.func {diff_func_style(&file1, &file2)}
        else {diff_loop_style(&file1, &file2)};
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

fn diff_func_style(file1: &[u8], file2: &[u8]) -> Vec<(usize,usize)> {
    let mut ranges: Vec<(usize,usize)> = vec![];
    let mut bool_enumed  = file1.iter()
        .zip(file2.iter())
        .enumerate();
    while let Some((start,_)) = bool_enumed.find(|&(_n, (l,r))| l!=r) {
        if let Some((stop,_)) = bool_enumed.by_ref()
            .find(|&(_n, (l,r) )| l==r ) {
                ranges.push((start,stop-1))
            };
    }
    ranges
}

fn diff_loop_style(file1: &[u8], file2: &[u8]) -> Vec<(usize,usize)> {
    /*
    Если байты различаются, а раньше были равны,
        то запомнить начало и установить флаг.
    Если байты равны, а раньше различались,
        то снять флаг и добавить в список пару (начало, конец).
    */
    let mut ranges: Vec<(usize,usize)> = vec![];
    let mut was_diff = false;
    let mut diff_start = 0;
    let counter = file1.iter()
        .zip(file2.iter())
        .enumerate()
        .map(|(counter,(l,r))| {
            if l == r {
                if was_diff {    
                    was_diff = false;
                    ranges.push((diff_start, counter-1))
                }
            } else if ! was_diff {
                diff_start = counter;
                was_diff = true
            };
        })
        .count();
    // Если оставались различия, они не сохранились,
    // добавляем от начала различий до конца файла.
    if was_diff {
        ranges.push((diff_start, counter))
    }
    ranges
}