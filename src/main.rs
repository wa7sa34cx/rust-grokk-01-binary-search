use std::cmp::Ordering;

fn main() {
    // searchable list
    let list = [1, 3, 5, 7, 9, 11, 13, 18, 19, 20, 21, 31, 55, 70, 82, 99];
    let item = 70;

    // println!("The index of {} is {}", item, binary_search(list, item));

    let mut low = 0;
    let mut high = list.len() - 1;
    // let mut mid = (low + high) / 2;

    // println!("{}", high);

    while low <= high {
        let mid = (low + high) / 2;

        match list[mid].cmp(&item) {
            Ordering::Less => {
                low = mid + 1;
            },
            Ordering::Greater => {
                high = mid - 1;
            },
            Ordering::Equal => {
                println!("The index of {} is {}", item, mid);
                break;
            }
        }
    }

}