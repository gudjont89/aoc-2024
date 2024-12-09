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
    0
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

    // #[test]
    // fn test_run_second() {
    //     assert_eq!(run_second(false), TBD);
    // }
}
