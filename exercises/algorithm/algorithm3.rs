/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM DONE

fn sort<T>(array: &mut [T])
where T:PartialOrd+Copy,
{
	//TODO
    //bubble sorting,ascending
    // for i in 0..array.len() {
    //     for j in 0..array.len() - 1 - i {
    //         if array[j] > array[j + 1] {
    //             array.swap(j, j + 1); // 直接交换，不需要 Copy
    //         }
    //     }
    // }

    //insert sorting - no clone
    // for i in 1..array.len(){
    //     let mut j=i;
    //     while j > 0 && array[j-1] > array[j]{
    //         array.swap(j-1,j);
    //         j-=1;   //move backwards
    //     }
    // }

    /*insert sorting - clone */
    for i in 1..array.len(){
        let current= array[i];
        let mut j=i;
        while j>0 && array[j-1] > current{
            array[j]=array[j-1];
            j-=1;
        }
        array[j]=current;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}