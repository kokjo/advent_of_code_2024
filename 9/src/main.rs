#![feature(linked_list_cursors)]

use std::collections::LinkedList;

fn part_1(input: &str) -> u64 {
    let mut inuse = true;
    let mut layout: Vec<Option<u64>> = vec![];
    let mut fileid = 0;
    for count in input.chars().filter_map(|d| d.to_digit(10).map(|x| x as usize)) {
        if inuse {
            layout.extend((0..count).map(|_| Some(fileid)));
            fileid += 1;
        } else {
            layout.extend((0..count).map(|_| None));
        }
        inuse = !inuse;
    }

    let mut i = 0;
    let mut j = layout.len() - 1;
    while i < j {
        while layout[i].is_some() { i += 1; }
        while layout[j].is_none() { j -= 1; }
        layout.swap(i, j);
    }

    layout.into_iter().filter_map(|x| x).enumerate().map(|(i, fileid)| (i as u64) * fileid).sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Chunk {
    InUse(u64, u64),
    Free(u64)
}

impl Chunk {
    fn len(&self) -> u64 {
        match *self {
            Chunk::InUse(size, _) => size,
            Chunk::Free(size) => size,
        }
    }

    fn is_free(&self) -> bool {
        match *self {
            Chunk::InUse(_, _) => false,
            Chunk::Free(_) => true,
        }
    }
}

fn part_2(input: &str) -> u64 {
    let mut inuse = true;
    let mut fileid = 0;
    let mut layout = LinkedList::new();
    for count in input.chars().filter_map(|d| d.to_digit(10).map(|x| x as u64)) {
        layout.push_back(if inuse {
            fileid += 1;
            Chunk::InUse(count, fileid-1)
        } else {
            Chunk::Free(count)
        });
        inuse = !inuse;
    }

    let mut final_layout = LinkedList::new();
    'outer_loop: while layout.len() != 0 {
        if let Some(chunk) = layout.pop_back() {
            match chunk {
                Chunk::InUse(_, _) => {
                    let mut cursor = layout.cursor_front_mut();
                    while let Some(cur_chunk) = cursor.current() {
                        if cur_chunk.is_free() && chunk.len() <= cur_chunk.len() {
                            let cur_chunk = cursor.remove_current().unwrap();
                            cursor.insert_before(chunk);
                            if cur_chunk.len() != chunk.len() {
                                cursor.insert_before(Chunk::Free(cur_chunk.len() - chunk.len()));
                            }
                            cursor.push_back(Chunk::Free(chunk.len()));
                            continue 'outer_loop;
                        }
                        cursor.move_next();
                    }
                    final_layout.push_front(chunk);
                },
                Chunk::Free(_) => {
                    final_layout.push_front(chunk);
                }
            }
        }
    }

    let mut checksum: u64 = 0;
    let mut offset: u64 = 0;
    for chunk in final_layout {
        checksum += match chunk {
            Chunk::InUse(size, fileno) => fileno * (offset..offset+size).sum::<u64>(),
            _ => 0
        };
        offset += chunk.len()
    }

    checksum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
//    println!("part_2 = {}", part_2("2333133121414131402"));
}
