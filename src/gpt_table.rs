use memmap::{Mmap, Protection};

pub struct GPTTable {
    disk_image: Mmap,
}

//Names are copied from Linux block/partitions/efi.h
//https://github.com/torvalds/linux/blob/9256d5a308c95a50c6e85d682492ae1f86a70f9b/block/partitions/efi.h#L70
#[repr(C)]
struct GPTHeader {
    signature: [u8; 8],
    revision: [u8; 4],
    header_size: i32,
    header_crc32: [u8; 4],
    reserved1: [u8; 4],
    my_lba: u64,
    alternate_lba: u64,
    first_usable_lba: [u8; 8],
    last_usable_lba: [u8; 8],
    guid: [u8; 16],
    partition_entry_lba: u64,
    num_partition_entries: u32,
    sizeof_partition_entry: u32,
    partition_entry_array_crc32: [u8; 4],
}

static GPT_SIGNATURE: [u8; 8] = [0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54];

impl GPTTable {
    fn new(disk_image_filepath: String) -> GPTTable {
        let disk_image = Mmap::open_path(disk_image_filepath,
                                         Protection::Read).unwrap();

        //TODO: Do some sanity check here.

        //Get [u8] slice.
        let image_slice = unsafe{ disk_image.as_slice(); };
        
        //Check magic.
//        image_slice 
        

        GPTTable {
            disk_image: disk_image,
        }
    }
}

#[test]
#[should_panic]
fn file_not_found_test() {
    let gpt_image = GPTTable::new("test_disk_image_not_exist.img".to_string());
}

#[test]
fn gpt_image_sanity_check() {
    //TODO: Do sanity check.
    let gpt_image = GPTTable::new("test_disk_image_gpt.img".to_string());
}
