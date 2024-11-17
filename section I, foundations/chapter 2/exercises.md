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
				















_2.1-1 using fig 2.2 as the model illustrate the operation of insert sort on an array containing 31 41 59 26 41 58... so perhaps in rust?...
! ctrl+f 'insertion_sort(a: &mut'
2.1-2 using the ps for sum-array on the next page that computes adding n numbers in arr a together... state a loop invariant for the procedure
and use init, maint, and term properties to show that SUM-ARRAY returns the sum of the numbers in a[1:n]
2.1-3 reverse the insertion-sort algo
2.1-4 write a linear searching algo and prove it's correctness in the invariant procedure
2.1-5 requires braining out...
given the problem of adding 2 n-bit binary integers a, and b...
...that are stored in two n-element arrays a[0:n -1], and b[0:n-1]...
where each element is either 0, or 1...
a = the sum of items constrained by (n-1, i=0) for each product of 2 raised by i...
and b is the same function... may be misreading that ACTUAL FUNC MATH NOTATION
the sum c = a + b of the two integers should be stored in a binary form in an (n+1)-elemtn array C[0:n]...
where c = ... similar constraints for summation except n, and i=0....
write a procedure ADD-BINARY-INTEGERS that takes A and B input arrays with length n
and returns array C holding the sum of A and B . . ?
2.1-5 finished going for walk and an hour and half before class...
gpt summary of the above which I pretty much understood... the sum of 2 arrays of bits
gpt analysis
two binary numbers a, and b
a[0:n-1] represents binary number a, in little endian format (least significant at 0)
b[0:n-1]: in the same format
... is LaTeX what wikipedia uses?
... OKAY I can read it
... sum of numbers between i and n-1 where i starts at 0...
for each index of array A, index * 2 ^ i
... the problem states prior to the summation function that a and b
are 2 n-bit binary numbers.. fall back
an integer n, which is the length of A and B that is considered
...
goal: write procedure add-binary-integers that takes input a, and b, along with n, and returns array C holding
the sum of a and b
other words: computer the binary sum c = a + b, and the result in array c,
since a and b each have n bits their sum could require up to n + 1 bits to handle any potential
overflow or carryin
the result should be stored in an n+1-element array, c[0:n], also in little endian format
binary addition...
each position i in the arrays a and b correspond to binary digit 0 or 1, and contribute...
a[i] * 2 ^ i and the same for b to the values of a and b, respectively
expected output:
the result should be in an array c of length n + 1???
this array will store the sum of the binary numbers in binary form
loop through each bit position i from 0 through n - 1:
add a[i], and b[i] - and any carry from the previous position....
determine the new c[i] (the sum mod 2) and update the carry for
the next position
handle the final carry
!!! solved this last night for both little and big endian binary addition 11/10/24
// moving on to 2.2!

e x e r c i s e s . . . . .

expression the function n^3 / 1000 + 100n^2 - 100n + 3 in terms of theta notation...

would it be O(n^3 + n^2) I wonder, or just O(n^3)...?

  

consider sorting n numbers in array a[1:n] by first finding the smallest element of a and exchanging it with the elemtn in a[1] - then find the

smallest element of a[2:n] and exchange it with a[2]... and so on.... continue in this manner for the first n-1 elements of a - write PS for

this algo... which is known as SELECTION SORT, what loop invariant does this algo maintain? why does it need to run for only

n-1 elements rather than for all n elements? give the worst case run time for selection sort theta notaiton -

is the best-case running time any better?

... my thoughts....

since this algo needs to sort through the entire subarray n-1 every single time to find the smallest that NO it is not any better

in fact in all but one case I believe it to be expressly worse than insertion sort where the one case where the input is backwards

entirely...

since it will be iterating as many times of this the theta notation would be O(n^2) again, however, the worst case

is ALWAYS the case where as with insertion sort it is not ALWAYS the worst case...

CLAUDE: VERY INSIGHTFUL, AND ACCURATE GREAT JOB FAM!

consider linear search again... how many elements of the input array need to be checked on average... assuming that the element is equally possibly in

any index in the array... and additionally how baout the worst case

in the average case O(n/2) since approximately half would be looked through on average, and in the worst case O(n) since the search index either isn't

present or is in the last index of the array

CLAUDE: EXACTLY!

how can any sorting algo be modified to have a good best-case run time?

check if it's sorted ahead of sorting

CLAUDE: GREAT! yes, that is a good idea