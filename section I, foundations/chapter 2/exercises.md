**2.1-1, illustration of insertion sort on the array**
```rust
fn insertion_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i as i32 - 1;
        
        // Insert key into the sorted part of the array
        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}
```

the exercise asks us to illustrate the operation of insertion sort on the sequence
	31, 41, 59, 26, 41, 58
		step-by-step
			1, starting from the second element (n = 2), and insert it into the sorted
				portion of the array which initially only contains 31
			2, move to 59, and insert it into the sorted portion which requires
				no change since 59 was greater than 41
			3, move 26 into the sorted part
				it will be inserted before 31
					this required all the inputs before the location 26
						was inserted at to be shifted to the right
			4, following this procedure, the rest of the items are considered for the arr
			
**2.1-2, loop invariant for sum-array**
```rust
fn sum_array(arr: &Vec<i32>, n: usize) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        sum += arr[i];
    }
    sum
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let result = sum_array(&arr, arr.len());
    println!("{}", result); // Output: 15
}
```

the exercise asks us to use loop invariant to explain the sum_array function
	loop invariant:
		init:
			before the loop begins sum is 0, and the sum of the first elements
				is 0 as there aren't any yet
		maintenance:
			at the start of each iteration each element for a[i] is added to the sum
				this maintains that for each loop the sum increases for the
					value of a[i] as each index is considered
		termination:
			when the loop terminates the sum of all the indexes is the value of sum
				which is the excepted solution for a problem instance

**2.1-3, insertion sort in decreasing order**
```rust
fn insertion_sort_decreasing(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i as i32 - 1;
        
        // Insert key into the sorted part of the array (decreasing order)
        while j >= 0 && arr[j as usize] < key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}
```

the exercise asks 
	to modify insert-sort to sort in monotonically decreasing order
		so change the comparison in the inner loop
			`while j >= 0 and A[j] < key: # Change comparison to '<'`
			
**2.1-4, linear search with loop invariant**
```rust
fn linear_search(arr: &Vec<i32>, x: i32) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == x {
            return Some(i);
        }
    }
    None // NIL
}
```

this exercise asks us to use loop invariants to describe the above linear search algo
	loop invariant:
		init:
			before starting the value of i is always 0, and this is always true
		maint:
				if x isn't found the invariant is maintained after checking a[i]
					if it's equal to x the search would have ended
						with Some(i) being returned from the function
		term:
			if the loop ends None is returned indicating no value matched in the array

**2.1-5: Adding Two Binary Integers**
```rust
fn add_binary_integers(a: &Vec<i32>, b: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut c = vec![0; n + 1];
    let mut carry = 0;

    for i in (0..n).rev() {
        let sum = a[i] + b[i] + carry;
        c[i + 1] = sum % 2;  // Store current bit
        carry = sum / 2;      // Update carry for next iteration
    }
    
    c[0] = carry; // If carry remains, place it in the most significant bit
    c
}
```

the exercise asks us to come up with a procedure for adding two binary integers that
	...takes two input arrays, and adds them together, to produce an array c
		...because this is in rust a vector is used to store c as
			... a vector because the size of the array isn't able to be known 
				... at compile time
				

***2.3-1 merge sort illustration***
	illustrate merge sort on 3, 41, 52, 26, 38, 57, 9, 49
		divide, divide the array in half
		conquer, sort each half recursively
		combine, merge the halves back together
			`3, 41, 52, 26`,       `38, 57, 9, 49`
				`3, 41,`      `52, 26,`       `38, 57,`     `9, 49`
					`3`  `41`  `52` `26`  `38` `57` `9` `49`
						`3, 41,`      `26, 52`      `38, 57`    `9, 49`
							`3, 26, 41, 52`    `9, 38, 49, 57`
		sorted: `3, 9, 26, 38, 41, 49, 52, 57`

***2.3-2 argument for merge-sort condition***
	in merge sort,  if p < r ensures that a recursive call is only valid when `p <= r`
		if p > r the sub array is empty so the func immediately terminates
			since the call ensures `n >= 1` then `1` is always less than `n`, and so
				then `p != r` can be expressed to say that the length of `r` has not
					decreased past the value of p
						_although this isn't the best idea_

***2.3-3 loop invariant for merge procedure***
	invariant for the while loop in the merge proc
		the while loop in merge ensures that at the start of each iteration
			... the left portion of the merged subarray is less than or
				...equal to the right portion
	specifically it maintains:
		the sub arrays are sorted
			the next element to be placed in the merged subarray is the smallest
				of each comparison
	to prove correctness:
		invariant
			init: the sub arrays are sorted, and the left is equal to or less than
				... the right array
			maintains:
				through each loop we maintain that the items are placed back into
					...the array in sorted order by incrementing through each array
						...and taking the lesser of each value
							...then finally when an array is empty is iterates
								...through the remaining array adding the items
									...that are already sorted back into the array
			termination:
				when the items are back in the provided array that the sorted items
					were placed into the procedure terminates having iterated through
						each item

***2.3-4 inductive proof for recurrence***
	for n <= 2, being a power of 2, the recur was
		`T(n) = 2T(n/2) + 2`
	base case:
		`T(2) = 2`
		
***2.3-5 recursive insertion sort***

```rust
fn insertion_sort_recursive(arr: &mut [usize]) {
    if arr.len() <= 1 {
        return;
    }
    let len = arr.len();
	insertion_sort_recursive(&mut arr[0..len - 1]);
    insert(arr);
}
fn insert(arr: &mut [usize]) {
    let len = arr.len();
    let mut i = len - 2;
    let last = arr[len - 1].clone();
    while arr[i] > last {
        arr[i + 1] = arr[i].clone();
        if i == 0 { break; }
        i -= 1;
    }
    arr[i + 1] = last;
}
```

`very happy to have covered this comparison to insertion sort`
	`it's made me think that I need to spent more time on`
		`... how exactly to solve more problems recursively`
			`just for more practice thinking this way`

***2.3-6 binary search***
	