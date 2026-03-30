use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;

pub fn devices() {
    println!("Found Nybbles:");
    for tty in find_nybble() {
        let sysfs = find_sysfs_node(&tty);
        let serial = read_sn(&sysfs);
        println!("{:15} {}", serial, tty.display())
    }
}

pub fn find_nybble() -> Vec<PathBuf> {
    find_tty()
        .into_iter()
        .filter(is_nybble) 
        .collect()
}

fn find_tty() -> Vec<PathBuf>{
    fs::read_dir("/dev")
        .expect("/dev not found?")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(is_tty)
        .collect()
}

fn is_tty(path: &PathBuf) -> bool {
    match path.file_name().and_then(|name| name.to_str()) {
        Some(str) => {
            str.starts_with("tty")
        },
        _ => false
    }
}

fn is_nybble(tty: &PathBuf) -> bool {
    let sysfs_path = find_sysfs_node(tty);
    let product_id = read_id(&sysfs_path.join("idProduct"));
    let vendor_id = read_id(&sysfs_path.join("idVendor"));

    match (product_id, vendor_id) {
        (Ok(product_id), Ok(vendor_id)) => {
            product_id == libnyb::PRODUCT_ID && vendor_id == libnyb::VENDOR_ID
        }
        _ => false,
    }

}

fn read_id(id_file: &Path) -> Result<u16> {
    let contents: String = fs::read_to_string(id_file)?;
    let id = u16::from_str_radix(&contents.trim(), 16)?;
    Ok(id)
}

fn read_sn(sysfs: &Path) -> String {
    let serial_file = sysfs.join("serial");
    fs::read_to_string(&serial_file)
        .expect(&format!("Unable to read {:?}", &serial_file))
        .trim()
        .to_string()
}

fn find_sysfs_node(tty: &Path) -> PathBuf {
    let tty_name = tty.file_name().unwrap();
    let sys_tty = PathBuf::from("/sys/class/tty").join(tty_name);

    let path = fs::canonicalize(&sys_tty)
        .expect(format!("Failed to cannonicalize {:?}", tty).as_ref());

    path.parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect(&format!("Unable to get grandparent of tty sysfs node {:?}", path))
        .to_path_buf()
}