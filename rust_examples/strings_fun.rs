/*  String objects in Rust are made of 3 parts:
    byte ptr into data buffer, length of bytes 
    stored in buffer, capacity of buffer.

    Q/ How does this buffer grow w.r.t. length?
    A/ Let's try to answer this below.
*/

fn add_char(s: &mut String) 
{
    s.push('X'); 
}

fn main() 
{
    // print an emoji bc I didn't know we could do this
    let heart = vec![240, 159, 146, 150];
    let heart_emoji = String::from_utf8(heart).unwrap(); 
    println!("Hello, world! {}", heart_emoji);
    
    // check how capacity grows
    let mut s = String::new();
    let mut curr_cap = 0;
    for _ in 0..17 {
        add_char(&mut s); // call subroutine for practice (note the &mut)
        if s.capacity() != curr_cap {
            println!("str={}, len={}, cap={}", s, s.len(), s.capacity());
            curr_cap = s.capacity();
        }
    }   
    // hmm.. seems like capacity doubles (i.e., 8, 16, 32...)

    // let's try an assert
    assert_eq!(heart_emoji, String::new()); // expected failure :)

    return ();
}
