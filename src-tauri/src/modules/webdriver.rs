use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use byteorder::{LE, ByteOrder};
use std::io::{Error, ErrorKind};
use std::str::from_utf8;
use crate::modules::game::Game;
use crate::modules::enums::Platform;

/// Returns a URL string from a array/vector of bytes.
/// # Arguments
/// * start (usize): Starting index of the URL in the data array.
/// * data (&[u8]): Array of bytes to read from.
fn read_url(start: usize, data: &[u8]) -> Result<String, Error> {
    //Get length of url
    let len = <LE as ByteOrder>::read_u16(&data[start..start+2]);
    //Get slice of data that is just our URL
    let url_bytes = &data[start+2..start+2+len as usize];

    //Convert to string
    let url = match from_utf8(url_bytes) {
        Ok(v) => v.to_owned(),
        Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
    };

    return Ok(url);
}



/// Returns the index of the first occurence of a subsequence in a sequence.
/// # Arguments
/// * subsequence (&Vec<u8>): Subsequence to search for.
/// * sequence (&Vec<u8>): Sequence to search in.
/// * rightmost (bool): Whether or not to search for the rightmost occurence of the subsequence.
fn find_subsequence(subsequence: &Vec<u8>, sequence: &Vec<u8>, rightmost: bool) -> Option<usize> {
    //Enumerate through sequence
    let mut seq = sequence.clone();
    let mut subseq = subsequence.clone();

    if rightmost {
        seq.reverse();
        subseq.reverse();
    }

    for (i, _) in seq.iter().enumerate() {
        //If the current index + the length of the subsequence is greater than the length of the sequence, return None.
        if i + subseq.len() > seq.len() {
            return None;
        }

        //If the current index + the length of the subsequence is less than the length of the sequence, check if the subsequence is present at the current index.
        if &seq[i..i+subseq.len()] == subseq {
            if rightmost {
                return Some(seq.len() - i - subseq.len());
            }
            return Some(i);
        }
    }

    return None;
}


/// Reads from a TcpStream into a byte array.
/// # Arguments
/// * stream (&mut TcpStream): TcpStream to read from.
/// * bytes (&mut [u8]): Byte array to read into.
async fn read_from_stream(stream: &mut TcpStream, bytes: &mut [u8]) -> Result<usize, Error> {
    let bytes_read: usize = match stream.read(bytes).await {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read from socket: {}", e.to_string());
            return Err(Error::new(ErrorKind::Other, e))
        },
    };

    return Ok(bytes_read);
}


/// Gets the patch urls from the patch server.
/// Ported from https://github.com/wizspoil/wizdiff/blob/374cd3262b02701c4d0067fd8e6b7612c9e4bb4f/wizdiff/webdriver.py#L18C18-L18C18
/// # Arguments
/// * game (&Game): Game to get patch urls for.
/// * platform (&Platform): Platform to get patch urls for.
pub async fn get_patch_urls(game: &Game<'_>, platform: &Platform) -> Result<(String, String), Error> {
    let url = format!("patch.us.{}.com:{}", game.name, platform.to_string());

    // Connect to the patch server
    let mut stream = TcpStream::connect(url).await?;

    //???
    let mut bytes = b"\x0D\xF0\x24\x00\x00\x00\x00\x00\x08\x01\x20".to_vec();
    let empty_bytes = vec![0u8; 29];
    bytes.extend_from_slice(&empty_bytes);

    stream.write_all(&bytes).await?;
    stream.flush().await?;

    //Session offer, do nothing with the bytes we read
    let _ = read_from_stream(&mut stream, &mut [0u8; 4096]).await?;

    //Read 4096 bytes from the stream
    let mut data = vec![0u8; 4096];
    let _ = read_from_stream(&mut stream, &mut data).await?;

    //Search for the URLs in the data read.
    let match_bytes = b"http".to_vec();
    let file_list_url_start = match find_subsequence(&match_bytes, &data, false) {
        Some(v) => v - 2,
        None => return Err(Error::new(ErrorKind::Other, "Unable to find file list url in patch string.")),
    };

    let base_url_start = match find_subsequence(&match_bytes, &data, true) {
        Some(v) => v - 2,
        None => return Err(Error::new(ErrorKind::Other, "Unable to find base url in patch string.")),
    };

    //Read the URLs from the data
    let file_list_url = read_url(file_list_url_start, &data)?;
    let base_url = read_url(base_url_start, &data)?;

    return Ok((file_list_url, base_url));

}