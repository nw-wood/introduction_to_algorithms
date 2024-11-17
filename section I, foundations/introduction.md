this part of the book gets the reader started with some of the mathematical tools to show
	...that algos are doing the right thing and doing it efficiently
		later parts will build upon this base understanding

chapter 1 provides and overview of algos; this chapter defines what they are
	it also makes the case for them as a technology
		... alongside other things like hardware, ux, oop and systems
			... as well as networks

chapter 2 we see the first algos
	sorting a sequence
		written in PS
			insertion sort comes first
				...which is incremental
					then merge sort
						...which is recursive 'divide and conquer'
							the rate of increased time required is different between
								... the two different algos
									in ch 2 asymptotic notation is defined
										... to express the difference

chapter 3 defines the notation
	it's used to bound the growth of functions
		most often this is used to describe the run-time of algos from
			...an upper and lower bound
				ch starts by informally defining the most common notations and giving
					... examples of their application
						then formal descriptions of five different asymptotic notations
							... and presents conventions for them
								the rest of the ch is for presentation of notation
									making sure to have the reader on track with it

chapter 4 delves into divide and conquer (dac) methodology
	...in ch 2
		it provides additional methods for using the dac method
			ch 4 contains solving recurrences
				useful for describing recurrences
					... these are useful for solving run times for recursive algos
						in the substitution method I'll guess an answer then prove it's
							... correct
								recursion trees provide a way to generate guesses
									ch 4 also presents the master method
										although the ch provides a proof for master
										method the reader should be okay
										employing the method without
										using the proof

ch 5 introduces probablistic analysis and randomized algos
	randomized algorithms can be used to put a bounds on the error rate of algorithms
		... that can mess up on limited cases
			an algos whos input or run time uses a random number generator would
				... be an example of one of these algorithms
					in some cases for inputs you can assume they adhere to the rules of
						some inherent probabilistic distribution and this information
							... can be used to optimize and change the behavior of one

appendices A-D refer to stuff that'll be useful to know when reading the book
	it may have been material in schooling, or outside of the context of the book, but is
		especially useful for it's interpretation
			all the chapters in part I of the appendix are in a tutorial-like form
				since the material in it the reader is unlikely to have had prior knowledge
					...of
