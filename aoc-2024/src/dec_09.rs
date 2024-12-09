use itertools::Itertools;

use crate::util::read_single_string_from_file;

pub fn run_first(is_real: bool) -> usize {
    let disk_map = read_single_string_from_file(is_real, 9, None);
    let mut memory_space_content_vector = parse_disk_map_to_memory_space_content_vector(disk_map);

    let (
        indexed_free_memory_spaces, 
        indexed_file_memory_spaces): 
        (Vec<(usize, &MemorySpaceContent)>, Vec<(usize, &MemorySpaceContent)>
    ) = memory_space_content_vector
        .iter()
        .enumerate()
        .partition(|(_, msc)| msc.is_free());
    
    let free_memory_space_index_iter = indexed_free_memory_spaces
        .into_iter()
        .map(|(index, _)| index);
    let file_memory_space_index_iter = indexed_file_memory_spaces
        .into_iter()
        .rev()
        .map(|(index, _)| index);
    let index_pairs = free_memory_space_index_iter
        .zip(file_memory_space_index_iter)
        .collect::<Vec<(usize, usize)>>();

    for (free_index, file_index) in index_pairs {
        if free_index > file_index {
            break;
        }

        memory_space_content_vector.swap(free_index, file_index);
    }

    memory_space_content_vector
        .iter()
        .enumerate()
        .map(|(index, value)| index * value.checksum_value())
        .sum()
}

pub fn run_second(is_real: bool) -> usize {
    let disk_map = read_single_string_from_file(is_real, 9, None);
    let mut memory_block_vector = parse_disk_map_to_memory_block_vector(disk_map);

    let before_total_memory_length = memory_block_vector.iter().map(|x| x.block_size).sum::<usize>();

    let file_memory_blocks = memory_block_vector
        .iter()
        .filter(|mb| !mb.block_type.is_free())
        .rev()
        .cloned()
        .collect::<Vec<MemoryBlock>>();

    for file_memory_block in file_memory_blocks {
        let free_memory_block = memory_block_vector
            .iter()
            .filter(|memory_block| memory_block.block_type.is_free())
            .filter(|free_memory_block| 
                free_memory_block.block_size >= file_memory_block.block_size &&
                free_memory_block.start_index < file_memory_block.start_index
            )
            .sorted()
            .next()
            .cloned();
        
        let free_memory_block = match free_memory_block {
            Some(fmb) => fmb,
            None => continue,
        };
        
        memory_block_vector.retain(|mb| mb.start_index != file_memory_block.start_index && mb.start_index != free_memory_block.start_index);
        memory_block_vector.append(&mut swap_memory_blocks(&free_memory_block, &file_memory_block));
    }

    let after_total_memory_length = memory_block_vector.iter().map(|x| x.block_size).sum::<usize>();
    assert_eq!(before_total_memory_length, after_total_memory_length);

    memory_block_vector
        .iter()
        .map(|x| x.calculate_checksum_value())
        .sum::<usize>()
}

fn parse_disk_map_to_memory_block_vector(disk_map: String) -> Vec<MemoryBlock> {
    let mut current_memory_index = 0;
    
    disk_map
        .chars()
        .enumerate()
        .filter_map(|(disk_map_index, block_size_char)| {
            let block_size = block_size_char.to_digit(10)?;
            let previous_memory_location = current_memory_index;
            current_memory_index = current_memory_index + block_size;

            if block_size == 0 {
                return None;
            } else {
                Some((disk_map_index, previous_memory_location, block_size))
            }
        })
        .map(|(disk_map_index, block_start_index, block_length)| MemoryBlock::new(disk_map_index, block_start_index as usize, block_length as usize))
        .collect::<Vec<MemoryBlock>>()
}

fn parse_disk_map_to_memory_space_content_vector(disk_map: String) -> Vec<MemorySpaceContent> {
    disk_map
        .chars()
        .enumerate()
        .filter_map(|(disk_map_location, cardinality_char)| {
            let cardinality = cardinality_char.to_digit(10)?;
            Some((disk_map_location, cardinality))
        })
        .flat_map(|(disk_map_location, cardinality)| disk_map_location_to_repeated_memory_space_contents(disk_map_location, cardinality))
        .collect::<Vec<MemorySpaceContent>>()
}

fn disk_map_location_to_repeated_memory_space_contents(disk_map_location: usize, cardinality: u32) -> Vec<MemorySpaceContent> {
    let is_free = disk_map_location % 2 == 1;

    let repeated_value = match is_free {
        true => MemorySpaceContent::Free,
        false => {
            let id = disk_map_location / 2;

            MemorySpaceContent::File(id)
        },
    };

    std::iter::repeat(repeated_value)
        .take(cardinality as usize)
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum MemorySpaceContent {
    File(usize), // ID
    Free,
}

impl MemorySpaceContent {
    fn is_free(&self) -> bool {
        match self {
            MemorySpaceContent::File(_) => false,
            MemorySpaceContent::Free => true,
        }
    }

    fn checksum_value(&self) -> usize {
        match self {
            MemorySpaceContent::File(v) => *v,
            MemorySpaceContent::Free => 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
struct MemoryBlock {
    block_type: MemoryBlockType,
    start_index: usize,
    block_size: usize,
}

impl MemoryBlock {
    fn new(disk_map_index: usize, start_index: usize, block_length: usize) -> Self {
        let is_free = disk_map_index % 2 == 1;

        let block_type = match is_free {
            true => MemoryBlockType::Free,
            false => {
                let id = disk_map_index / 2;

                MemoryBlockType::File(id)
            },
        };

        Self { block_type, start_index, block_size: block_length }
    }

    fn calculate_checksum_value(&self) -> usize {
        match self.block_type {
            MemoryBlockType::File(id) => id * (self.start_index..(self.start_index + self.block_size)).sum::<usize>(),
            MemoryBlockType::Free => 0,
        }
    }
}

impl PartialOrd for MemoryBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_index.partial_cmp(&other.start_index)
    }
}

impl Ord for MemoryBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_index.cmp(&other.start_index)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
enum MemoryBlockType {
    File(usize), // ID
    Free
}

impl MemoryBlockType {
    fn is_free(&self) -> bool {
        match self {
            MemoryBlockType::File(_) => false,
            MemoryBlockType::Free => true,
        }
    }
}

fn swap_memory_blocks(
    free_memory_block: &MemoryBlock,
    file_memory_block: &MemoryBlock,
) -> Vec<MemoryBlock> {
    assert!(free_memory_block.block_type.is_free());
    assert!(free_memory_block.block_size >= file_memory_block.block_size);

    let swapped_free_memory_block = MemoryBlock { 
        block_type: MemoryBlockType::Free, 
        start_index: file_memory_block.start_index, 
        block_size: file_memory_block.block_size 
    };

    let file_id = match file_memory_block.block_type {
        MemoryBlockType::File(id) => id,
        MemoryBlockType::Free => panic!("Could not retrieve file ID"),
    };

    let swapped_file_memory_block = MemoryBlock {
        block_type: MemoryBlockType::File(file_id),
        start_index: free_memory_block.start_index,
        block_size: file_memory_block.block_size,
    };

    let block_size_difference = free_memory_block.block_size - file_memory_block.block_size;

    if block_size_difference == 0 {
        return vec![swapped_file_memory_block, swapped_free_memory_block]
    }

    let additional_free_memory_block = MemoryBlock {
        block_type: MemoryBlockType::Free,
        start_index: free_memory_block.start_index + file_memory_block.block_size,
        block_size: block_size_difference,
    };

    vec![swapped_file_memory_block, additional_free_memory_block, swapped_free_memory_block]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 1928);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 6301895872542);
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 2858);
    }

    #[test]
    fn test_memory_block_calculate_checksum() {
        let memory_block = MemoryBlock {
            block_type: MemoryBlockType::File(4),
            start_index: 3,
            block_size: 5,
        };

        let expected_result = 100; // 12 + 16 + 20 + 24 + 28
        let result = memory_block.calculate_checksum_value();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn real_run_second() {
        assert_eq!(run_second(true), 6323761685944);
    }
}
