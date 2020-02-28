use std::io;
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::enums::RegDisposition::REG_OPENED_EXISTING_KEY;
use winreg::RegKey;

const CHROME_KEY: &str = "ChromeHTML\\shell\\open\\command";
const INCOGNITO_DEFAULT: &str = "\"C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe\" -incognito  -- \"%1\"";

fn main() -> io::Result<()> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    println!("Opening {} in HKEY_CLASSES_ROOT...", CHROME_KEY);
    
    let (open_chrome, disp) = hkcr.create_subkey(CHROME_KEY)?;
    if let REG_OPENED_EXISTING_KEY = disp {
        println!("Updating default key to {}", INCOGNITO_DEFAULT);
        open_chrome.set_value("",&INCOGNITO_DEFAULT)?;
        println!("Done!");
    } else {
        println!("Key does not exist!");
    }
    
    Ok(())
}
