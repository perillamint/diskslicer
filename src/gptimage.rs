use memmap::{Mmap, Protection};

pub struct GPTImage {
    disk_image: Mmap,
}

static GPT_MAGIC: [u8; 8] = [0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54];

impl GPTImage {
    fn new(disk_image_filepath: String) -> GPTImage {
        let disk_image = Mmap::open_path(disk_image_filepath,
                                         Protection::Read).unwrap();

        //TODO: Do some sanity check here.

        GPTImage {
            disk_image: disk_image,
        }
    }
}
