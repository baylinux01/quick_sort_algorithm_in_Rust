fn main() 
{
    let mut arr0:Vec<isize>=Vec::new();

    arr0.push(65);
    arr0.push(63);
    arr0.push(64);
    arr0.push(89);
    arr0.push(92);
    arr0.push(88);
    arr0.push(83);
    arr0.push(-1);
    arr0.push(-5);
    arr0.push(-3);
    arr0.push(0);
    println!("Before quick sort: {:?}",arr0);
    let size_of_arr0= arr0.len();
    quick_sort(&mut arr0,0,size_of_arr0-1);
    println!("After quick sort: {:?}",arr0);
}
fn quick_sort(mut arr1: &mut Vec<isize>,start_index: usize,end_index:usize) 
{
    if start_index >= end_index
    {
        return;
    }
    let pivot=end_index;
    let mut i:usize = start_index; 
    let mut j:usize=start_index;
    while i <= pivot
    {
        if arr1[i] < arr1[pivot] && i<pivot
        {
            arr1.swap(i, j);
            j += 1;
        }
        if i==pivot
        {
            arr1.swap(j, pivot);
        }
        i+=1;
    }
    {
        quick_sort(&mut arr1,start_index,j-1);
    }
    {
        quick_sort(&mut arr1,j+1,end_index);
    }
     
    


}
