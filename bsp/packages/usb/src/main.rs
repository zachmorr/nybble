fn main() {
    println!("Hello, world!");
    usb_gadget::remove_all().expect("Failed to remove all gadgets")
}
