use std::io::Cursor;
use serde_json;
use upcaseinfo::info::UpcaseInfo;


#[test]
fn test_info_text() {
    // Read file into byte slice (&'static [u8; N])
    let info_buffer = include_bytes!("../testdata/Info");
    // Create a curser for the byte slice so Read is implemented
    let info_buffer_cursor = Cursor::new(info_buffer);

    // Pass the cursor to the UpcaseInfo::new to parse out the upcase info struct
    let upcase_info = UpcaseInfo::new(info_buffer_cursor)
        .expect("Error parsing Upcase:Info buffer.");
    
    // Serialize it to a JSON string.
    let upcase_as_text = upcase_info.to_string();

    assert_eq!(
        upcase_as_text, 
        "crc: 15770619046507800844\nosmajor: 10\nosminor: 0\nbuild: 16299\npackmajor: 0\npackminor: 0"
    );

    println!("UpcaseInfo text -> {}", upcase_as_text);
}


#[test]
fn test_info_json() {
    // Read file into byte slice (&'static [u8; N])
    let info_buffer = include_bytes!("../testdata/Info");
    // Create a curser for the byte slice so Read is implemented
    let info_buffer_cursor = Cursor::new(info_buffer);

    // Pass the cursor to the UpcaseInfo::new to parse out the upcase info struct
    let upcase_info = UpcaseInfo::new(info_buffer_cursor)
        .expect("Error parsing Upcase:Info buffer.");
    
    // Serialize it to a JSON string.
    let upcase_json = serde_json::to_string(&upcase_info)
        .expect("Error serializing upcase_info into JSON");

    assert_eq!(
        upcase_json, 
        r#"{"len":32,"filler":0,"crc":15770619046507800844,"osmajor":10,"osminor":0,"build":16299,"packmajor":0,"packminor":0}"#
    );

    println!("UpcaseInfo json -> {}", upcase_json);
}


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