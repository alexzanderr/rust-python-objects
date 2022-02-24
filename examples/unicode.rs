

use std::ops;
use std::ops::Index;
use std::string;

use unicode_segmentation::UnicodeSegmentation;
use color_backtrace;

fn iterate_over_unicode() {
    for g in "asd asdasd".graphemes(true) {
        println!("{}", g)
    }
}


// trait GetItem {
//     type Item;
//     fn index(&self, _index: usize) -> Option<Self::Item> ;
// }

// impl ops::Index<i32> for string::String {
//     type Output = char;
//     fn index(&self, _index: usize) -> Option<Self::Output> {
//         self.chars().nth(_index)
//     }
// }


fn main() {
    color_backtrace::install();



    let string = String::from("📦 📦 📦");
    // println!("{:?}", string.get(0));
    println!("{}", &string[0..4]);
    println!("{:?}", string.chars());
    println!("{:?}", string.chars().nth(2));

    // println!("{:?}", string[0]);
    // println!("{:?}", string.index(0));


}