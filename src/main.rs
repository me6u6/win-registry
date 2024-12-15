use windows_registry::*;

fn main() -> Result<()> {
    let key = CURRENT_USER.keys()?;
    println!("{:#?}", CURRENT_USER);
    for k in key {
        println!("----- {:#?}", k);
        let a = CURRENT_USER.open(k)?;
        let b = a.keys()?;
        for c in b {
            println!("{:#?}", c);
        }
        let d = a.values()?;
        for e in d {
            println!("key: {:#?}", e.0);
            let mut f = String::from_utf16_lossy(e.1.as_wide());
            f.retain(|c| c != '\0');
            println!("value: {:?}", f);
        }
    }

    Ok(())
}