//What happens under the hood
//How to read/write/seek on a file
//Simulating using Cursor
//Cursor is like a in-memory file on which you can apply all the operations as you can apply on a real file
//IT helps  to simulation the operation on a file in memory.


use std::io::prelude::*;
use std::io::{self,SeekFrom,Cursor};
use std::fs::File;

//Function to write bytes and print positions
fn write_ten_bytes_at_end<W: Write + Seek>(writer: &mut W) -> io::Result<()>{

    //Getting initial position before seeking
    let initial_position = writer.stream_position()?;
    println!("Initial Position : {}",initial_position);

    let new_pos = writer.seek(SeekFrom::End(-10))?;
    println!("After Seeking -10 from end, new position : {}",new_pos);

    for i in 0..10{
        writer.write(&[i])?;
        let current_position = writer.stream_position()?;
        println!("Wrote byte {} at position {}",i,current_position);
    }

    Ok(())
}

fn main() -> io::Result<()>{
    println!("Testing with in memory Cursor");
    //Creating a cursor with 15 zeroes
    let mut buff = Cursor::new(vec![0; 15]);
    println!("Created a buffer with size {}",buff.get_ref().len());

    //Writing bytes using our function
    write_ten_bytes_at_end(&mut buff)?;

    //Displaying the contents of the buffer
    println!("/n Final buffer contents: {:?}",buff.get_ref());
    println!("Last 10 elements : {:?}", &buff.get_ref()[5..15]);

    println!("/n ==== Testing with real file");
    let mut file = File::create("foo.txt")?;

    //Writing some intial data
    file.write_all(&[1;15])?;
    println!("Created file with 15 bytes of initial data");

    //Seek back to write our test bites.
    file.seek(SeekFrom::Start((0)))?;
    write_ten_bytes_at_end(&mut file)?;

    //Read and display file contents
    let content = std::fs::read("foo.txt")?;
    println!("File contents: {:?}",content);
    println!("Last 10 elements: {:?}",&content[5..15]);


    Ok(())
}

#[test]
fn test_writes_bytes(){
    let mut buff = Cursor::new(vec![0; 15]);
    write_ten_bytes_at_end(&mut buff).unwrap();
    assert_eq!(&buff.get_ref() [5..15], &[0,1,2,3,4,5,6,7,8,9]);
}