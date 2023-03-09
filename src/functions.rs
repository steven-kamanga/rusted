//Functions
//declare with pub keyword in front of it to make it public
//default they are private
fn count_up_to(x: i32) -> i32 {
    let mut _counter = 0;
    for i in 1..x + 1 {
        _counter += i;
    }
    _counter
}

fn pass_by_reference(a: &Vec<i32>) -> &Vec<i32> {
    a
}

fn main() {
    let x = 12;
    let value = count_up_to(x);
    let a = [2, 3, 4, 5, 6];
    // pass_by_reference(&a);
    //can also pass a vector
    let v1 = (0..50).collect::<Vec<i32>>();
    pass_by_reference(&v1);
    //the variables can be borrowed out of the scope of one function to another
    //But a referenced value can be safely returned (hahah made that up, I skipped that boring tutorial)
    print!("{:#?}", v1);
}


