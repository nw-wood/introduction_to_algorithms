***1.1 algorithms***

_1.1-1
Describe your own real-world example that requires sorting. Describe one that
requires finding the shortest distance between two points._

	deck of cards
	calculation to determine the optimal trajectory of a shot on a moving target

_1.1-2
Other than speed, what other measures of efficiency might you need to consider in
a real-world setting?_

	space requirements in memory, or on a hard drive

_1.1-3
Select a data structure that you have seen, and discuss its strengths and limitations._

	vectors
		they are fantastic for indexing values of and iterating over
			they are not as memory efficient as an array, which occupies
				... an unchanging space in memory

_1.1-4
How are the shortest-path and traveling-salesperson problems given above similar?
How are they different?_

	they both interact with distances between places and are gear toward
		...solving for efficiency or a shortest distance
	one solves between two points, and traveling-salesperson solves for many

_1.1-5
Suggest a real-world problem in which only the best solution will do. Then come
up with one in which <approximately= the best solution is good enough._

	exact measurements of materials handled by a robot that pours chemicals
		...identifying quantity jelly beans in a jar by an image

_1.1-6
Describe a real-world problem in which sometimes the entire input is available
before you need to solve the problem, but other times the input is not entirely
available in advance and arrives over time._

	gotta sort a vector that is already in memory
	some user input a socket is waiting on in an ajax implementation

***1.2 algorithms as a technology***

_1.2-1
Give an example of an application that requires algorithmic content at the applica-
tion level, and discuss the function of the algorithms involved._

	this text editor I'm using is doing all sorts of stuff to quickly
		...interact with this markdown text... it renders it... takes the
			...input... lots of stuff is happenin'

_1.2-2
Suppose that for inputs of size n on a particular computer, insertion sort runs in 8n2
steps and merge sort runs in 64 n lg n steps. For which values of n does insertion
sort beat merge sort?_

	as soon as n squared exceeds log of n, so very quickly
	8 * n * n < 64 * n * log(n)
	divide both sides by 8
	n * n < 8 * n * log(n)
	divide both sides by n (for n > 0)
	n < 8 * log(n)
	approximate by testing values of n
	30 < 8 * log(30) ~ 8 * (3.401) = 27.208 = FALSE
	so in this instance values of about 30 or less to sort are faster with insert
	but values of n greater than 30 would be faster with merge sort
	
_1.2-3
What is the smallest value of n such that an algorithm whose running time is 100n^2
runs faster than an algorithm whose running time is 2n on the same machine?_

	100*15^2 < 2^15
	22500 < 32768... TRUE
	100*14^2 < 2^14
	19600 < 16385... FALSE