use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::io::{Error, Write};
use std::str::from_utf8;


fn read_url(start: usize, data: &Vec<u8>) -> Result<String, Error> {
    // Read the length of the string by taking a slice of data from index start to start + 2 into an array with a capacity of 2.
    let str_len = u16::from_le_bytes([data[start], data[start + 2]]);
    let str_data = &data[start+2..start+2+str_len as usize];
    //Convert the bytes to a string slice.
    let url = match from_utf8(str_data) {
        Ok(v) => v.to_owned(),
        Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e)),
    };

    

    return Ok(url);
}

//Retu

//fix the return type
pub async fn get_patch_urls() -> Result<String, Error> {
    // Connect to a peer
    let mut stream = TcpStream::connect("patch.us.wizard101.com:12500").await?;

    let mut bytes = b"\x0D\xF0\x24\x00\x00\x00\x00\x00\x08\x01\x20".to_vec();
    let empty_bytes = vec![0u8; 29];
    bytes.extend_from_slice(&empty_bytes);

    println!("{:?} - {}", bytes, bytes.len());

    stream.write_all(&bytes).await?;
    stream.flush().await?;

    //Read 4096 bytes from the stream into data.
    let mut data = vec![0u8; 4096];
    let _ = match stream.read(&mut data).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to read from socket: {}", e.to_string());
            return Err(Error::new(std::io::ErrorKind::Other, e))
        },
    };

    // let file_list_url_start = match find_subsequence("http", &data) {
    //     Some(v) => v,
    //     None => return Err(Error::new(std::io::ErrorKind::Other, "Unable to find file list url in patch string.")),
    // };

    // let base_url_start = match find_subsequence_rightmost("http", &data) {
    //     Some(v) => v,
    //     None => return Err(Error::new(std::io::ErrorKind::Other, "Unable to find base url in patch string.")),
    // };
    

    // let file_list_url = read_url(file_list_url_start, &data)?;
    // let base_url = read_url(base_url_start, &data)?;
    // println!("{}", file_list_url);
    // println!("{}", base_url);
    
    return Ok("".to_string());

}