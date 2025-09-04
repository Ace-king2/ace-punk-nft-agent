use serde::{Deserialize, Serialize};
use utils::log;
mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[no_mangle]
pub extern "C" fn run() {
    log("This is a log inside the WASM module!");
    
    let input = utils::read_input();
    let messages = utils::parse_messages(&input);
    let system_message = utils::system_message("Your name is UOMI Agent".to_string());
    let modified_messages = utils::process_messages(system_message, messages);
    
    let cid = "bafkreicevizwv5glcsuhsqzokpowk4oh7kn4zl5xl5eiewjgfvxkhjgzdm".as_bytes().to_vec();
    let message_file = utils::get_cid_file_service(cid);
    log(&format!("Message from a file on IPFS: {:?}", String::from_utf8(message_file).unwrap()));
    
    let message_file_bytes = utils::get_input_file_service();
    log(&format!("Message from input file: {:?}", String::from_utf8(message_file_bytes).unwrap()));
    
    // Dynamic NFT responses based on input
    let input_str = String::from_utf8_lossy(&input);
    let response = if input_str.contains("CryptoPunks") {
        b"CryptoPunks floor: ~30 ETH".to_vec()
    } else if input_str.contains("Bored Ape") {
        b"BAYC floor: ~15 ETH".to_vec()
    } else {
        b"NFT agent ready. Ask about specific collections.".to_vec()
    };
    
    utils::save_output(&response);
}
