//Frigging pointers

fn main() {
    //store addresses of other variables *why tffffff?*
    let mut a = 7;

    unsafe {
        //let ptr = &a;//reference
        // let p = &a as *const i32; //raw ponter *Fncy: Dereferencing operator*
        let p = &mut a as *mut i32; //raw ponter *Fncy: Dereferencing operator*
        *p = *p + 10;
        println!("Pointer value: {:#?}", *p);
        println!("Pointer Address: {:x?}", p);
        println!("Address of a: {:x?}", std::ptr::addr_of!(a));
    }
    // print!("{a}");
}
