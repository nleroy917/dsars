mod list;

use list::LinkedList;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut list: LinkedList<u8> = LinkedList::new();

    list.append(1)?;
    list.append(2)?;
    list.append(3)?;
    list.append(4)?;
    list.push(0)?;

    println!("{}", list);

    Ok(())
    
}
