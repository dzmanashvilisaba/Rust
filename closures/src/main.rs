/*                                                                    *
 *                              PART:   I                             *
 *                                                                    */
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>, ) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}



/*                                                                    *
 *                              PART:   II                            *
 *                                                                    */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
    let store = Inventory {shirts: vec![ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue, ],};

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}",user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",user_pref2, giveaway2);

    

    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    //let n = example_closure(5);

    

    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
    let outer_var = 42;
    
    // A regular function can't refer to variables in the enclosing environment
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
    /*  PRINTED
        closure_annotated: 43
        closure_inferred: 43
        closure returning one: 1
    */


    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    only_borrows();
    println!("After calling closure: {:?}", list);
    /* PRINTED:
        Before defining closure: [1, 2, 3]
        Before calling closure: [1, 2, 3]
        From closure: [1, 2, 3]
        From closure: [1, 2, 3]
        After calling closure: [1, 2, 3]
    */



    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    /* PRINTED:
        Before defining closure: [1, 2, 3]
        After calling closure: [1, 2, 3, 7, 7] 
    */



    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
    /*Using "move" to force the closure for the thread to take
    ownership of "list" */
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || {println!("From thread: {:?}", list)}).join().unwrap();
    /* PRINTED:
        Before defining closure: [1, 2, 3]
        From thread: [1, 2, 3] 
    */


/********************************************************************** 
 *                                                                    *
 *                              PART:   II                            *
 *                                                                    *
 **********************************************************************/
    println!("\nMoving Captured Values Out of Closures and the
    Fn Traits\n");
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    /* PRINTED:
        [
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
            Rectangle {
                width: 10,
                height: 1,
            },
        ]
    */



    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
/*     println!("\nMoving Captured Values Out of Closures and the
    Fn Traits 22\n");

    let mut sort_operations = vec![];
    let value = String::from("by key called");
    list.sort_by_key(|r| {sort_operations.push(value); r.width});
    println!("{:#?}", list);*/



    /***************************************************
     *****************     EXAMPLE     *****************
     ***************************************************/
    println!("\nMoving Captured Values Out of Closures and the
    Fn Traits 333\n");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {num_sort_operations += 1;r.width});
    println!("{:#?}, sorted in {num_sort_operations} operations",list);


/********************************************************************** 
 *                                                                    *
 *            The Iterator Trait and the next Method                  *
 *                                                                    *
 **********************************************************************/
    




}
/* 
fn add_one_v1 (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x| { x + 1 };
let add_one_v4 = |x| x + 1 ;


The first line shows a function definition and the second line shows
a fully annotated closure definition. In the third line, we remove the
type annotations from the closure definition. In the fourth line, we
remove the curly brackets, which are optional because the closure
body has only one expression. These are all valid definitions that will
produce the same behavior when they’re called. The add_one_v3 and
add_one_v4 lines require the closures to be evaluated to be able to
compile because the types will be inferred from their usage.
*/


