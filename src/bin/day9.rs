use std::{cmp::min, collections::LinkedList};

use adventofcode_2024::runner;
use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SegmentType {
    Free,
    Filled(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Segment {
    len: i64,
    segment_type: SegmentType,
}

impl Segment {
    fn id(&self) -> Option<i64> {
        match self.segment_type {
            SegmentType::Filled(id) => Some(id),
            _ => None,
        }
    }
}

#[allow(dead_code)]
fn vis<'a, T>(segments: T)
where
    T: IntoIterator<Item = &'a Segment>,
{
    for segment in segments {
        match segment.segment_type {
            SegmentType::Free => {
                (0..segment.len).for_each(|_| {
                    print!(".");
                });
            }
            SegmentType::Filled(id) => {
                (0..segment.len).for_each(|_| {
                    print!("{id}");
                });
            }
        }
    }
    println!();
}

fn checksum<'a, T>(segments: T) -> i64
where
    T: IntoIterator<Item = &'a Segment>,
{
    let mut count = 0;
    let mut sum = 0;
    for segment in segments {
        match segment.segment_type {
            SegmentType::Free => {
                count += segment.len;
            }
            SegmentType::Filled(id) => {
                (count..count + segment.len).for_each(|num| {
                    sum += num * id;
                });
                count += segment.len;
            }
        }
    }
    sum
}

fn parse_input(input: &str) -> LinkedList<Segment> {
    let mut list = LinkedList::new();

    let mut toggle = true; // toggles between file and free space, start with file
    let mut id_counter = 0;

    for len in input
        .lines()
        .flat_map(|line| line.chars())
        .map(|x| x.to_string().parse::<i64>().unwrap())
    {
        if len == 0 {
            // nop
        } else {
            let segment = Segment {
                len,
                segment_type: if toggle {
                    id_counter += 1;
                    SegmentType::Filled(id_counter - 1)
                } else {
                    SegmentType::Free
                },
            };
            list.push_back(segment);
        }
        toggle = !toggle;
    }

    list
}

fn part1(input: &str) {
    let mut list = parse_input(input);
    let mut new_list = LinkedList::new();
    new_list.push_back(list.pop_front().unwrap());

    while !list.is_empty() {
        let back_segment = list.pop_back().unwrap();
        if back_segment.segment_type == SegmentType::Free {
            continue;
        }

        // back_segment is now guaranteed to not be a free segment

        if let Some(front_segment) = list.pop_front() {
            if !(front_segment.segment_type == SegmentType::Free) {
                // front_segment is not a free segment, transfer it to the new list
                new_list.push_back(front_segment);
                // and add the back segment back to og list, so we can process it next
                list.push_back(back_segment);
                continue;
            }

            // calculate how much we can write from og list to new list, and write it
            let overlap = min(front_segment.len, back_segment.len);
            let new_segment = Segment {
                len: overlap,
                segment_type: back_segment.segment_type,
            };
            new_list.push_back(new_segment);

            // calculate the leftover segments, and put them back into the original list
            if front_segment.len > overlap {
                list.push_front(Segment {
                    len: front_segment.len - overlap,
                    segment_type: SegmentType::Free,
                });
            }
            if back_segment.len > overlap {
                list.push_back(Segment {
                    len: back_segment.len - overlap,
                    segment_type: back_segment.segment_type,
                });
            }
        } else {
            // back_segment was the last segment, transfer it to the new list
            new_list.push_back(back_segment);
        }
    }

    println!("Day 9 Part 1: {}", checksum(&new_list));
}

fn part2(input: &str) {
    let mut vec = parse_input(input).into_iter().collect_vec();
    let file_ids: HashSet<i64> = vec
        .iter()
        .filter_map(|x| match x.segment_type {
            SegmentType::Filled(id) => Some(id),
            _ => None,
        })
        .collect();

    let mut locked_ids: HashSet<i64> = HashSet::new();
    while file_ids != locked_ids {
        let unlocked_file_idx = vec
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, x)| match x.segment_type {
                SegmentType::Filled(id) => {
                    if !locked_ids.contains(&id) {
                        Some(i)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .unwrap();
        let unlocked_file = vec[unlocked_file_idx];

        // look for free segment, that can fit our file, and is to the left of the file
        if let Some(free_space_idx) = vec.iter().enumerate().position(|(idx, seg)| {
            seg.segment_type == SegmentType::Free
                && seg.len >= unlocked_file.len
                && idx < unlocked_file_idx
        }) {
            // remove file and replace with free space
            vec[unlocked_file_idx] = Segment {
                len: unlocked_file.len,
                segment_type: SegmentType::Free,
            };

            // there is a free space that can fit our file; insert it there
            let free_space = vec[free_space_idx];
            vec[free_space_idx] = unlocked_file;

            // in case our free space was bigger than the segment we just moved, add the leftover free space back
            if free_space.len > unlocked_file.len {
                vec.insert(
                    free_space_idx + 1,
                    Segment {
                        len: free_space.len - unlocked_file.len,
                        segment_type: SegmentType::Free,
                    },
                );
            }
        }
        locked_ids.insert(unlocked_file.id().unwrap());
    }

    println!("Day 9 Part 2: {}", checksum(&vec));
}

fn main() {
    runner(part1);
    runner(part2);
}
