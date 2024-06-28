mod list;

use list::LinkedList;

fn main() {
    let mut list: LinkedList<u8> = LinkedList::new();

    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);
    list.push(0);

    println!("{}", list);
}
