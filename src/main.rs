mod test;
mod test1;



fn test_main1(){
    // mod test
    test::a::add();
    test::b::add();
    test::a::a::add();

    // mod test1
    test1::c::aaa::add();
}

fn main() {
    println!("hello rust");

    test_main1();
}