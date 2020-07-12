use std::io::Cursor;
use upcaseinfo::info::UpcaseInfo;


#[test]
fn test_info_parsing() {
    // Read file into byte slice (&'static [u8; N])
    let info_buffer = include_bytes!("../testdata/Info");
    // Create a curser for the byte slice so Read is implemented
    let info_buffer_cursor = Cursor::new(info_buffer);

    // Pass the cursor to the UpcaseInfo::new to parse out the upcase info struct
    let upcase_info = UpcaseInfo::new(info_buffer_cursor)
        .expect("Error parsing Upcase:Info buffer.");
    
    // Validate the known values
    assert_eq!(upcase_info.len, 32);
    assert_eq!(upcase_info.filler, 0);
    assert_eq!(upcase_info.crc, 15770619046507800844);
    assert_eq!(upcase_info.osmajor, 10);
    assert_eq!(upcase_info.osminor, 0);
    assert_eq!(upcase_info.packmajor, 0);
    assert_eq!(upcase_info.packminor, 0);

    println!("UpcaseInfo struct -> {:#?}", upcase_info);
}