










































fn main() {
        println!("an exploration of the many types of algorithms introduced in the book 'introduction to algorithms 4th edition'");
        println!("this is a compendium of tests written in rust to get hands-on with the knowledge this book offers");
        println!();
/* I N T R O D U C T I O N   T O   A L G O R I T H M S   4 T H   E D I T I O N   N O T E S
        SEE WHAT CAN BE ACCOMPLISHED!

        C H A P T E R - 1
        ================= 1.1 - A L G O R I T H M S

                an algorithm is a series of steps to take an input and turn it into an output                   

                they are tools for solving computational problems
                1) when the context of a problem instance is known they are often referred to as just problems
                the statement of the problem specifies the general terms the 
                        desired input and ouput relationship should have

                algo's describe the procedure for achieving a proper io relationship for all
                        instances of problems

                ex; sorting a list of numbers that increase monotonically
                        this is a common issue and is grounds for using design and analysis tools

                ex;
                        input: sequence n (A1, A2, ...., An)
                        output: a permutation of (A'1, A'2, .... A'n) such that
                                A'1 <= A'2 <= ... <= A'n
                        given: (31, 41, 59, 26, 41, 58)         
                        expected: (26, 31, 41, 41, 58, 59)

                a given input in this manor is an instance of a sorting problem

                many programs use sorting as an in-between step it is fundamental in programming

                there are many sorting algos, and which is best depends on a lot of factors
                        # of itmes
                        extent of sorting required dependong the context
                        possible restrictions on item values
                        computer architecture
                        storage devices being used: dram, hdd, ssd's, or even tapes

                an algo is correct if when provided with any input it halts, finishes computing in finite time,
                and outputs a solution... finite time being a limited duration of time

                a correct algo solves the problem instance

                an incorrect algo may not halt at all on some inputs
                        or may provide an incorrect answer

                contrary to ordinary expectations incorrect algos have uses if you can control error rates

                algos can be specified by language, program, or hardware with the only requirement

                there's definitely more algos than just sorting
                        - human genome project: determining the sequences makiong up the human genone 
                                roughly 3 billion chemical base pairs that make up dna
                                        storing this info, and tools of data analysis would use algos
                        - determining similarily of the sequences is a great example
                        - the internet enables people all over to get and send info
                                with clever algos, sites can manipulate more data faster
                                examples of probs that make use of algos include:
                                        route finding
                                        text searching for info
                        - ecommerce goods and services negotiation electronically
                                privacy information such a cc's, passwords, bank statements
                                public key crypto
                                digital signatures
                                        all use number theory and algos
                        - manufacturing
                                allocation of scarce resources
                                        where to place oil wells to maximize profits
                                political campaigns may want to determine where to spend money
                                how airlines assign crews and determine flight patterns
                                        even more so to use algos to adhere to regulations
                                isp's may want to know ideal geographic locations to place resources
                        
                        all of these problems can be solved with linear algos explored in 29

                here's a specific problem:
                        on a road map there is a distance between each pair of adjacent intersections that are marked
                        we want to determine the shortest distance from one intersection to another
                        the number of possible routes is very large even disallowing for routes that overlay
                        how do you choose which of all routes is shortest?
                                can start by modeling the map (which is itself a model of actual roads)
                                as a graph (which we meet in part 6 and appendix b)
                                        in this graph you wish to find the shortest path from one
                                        vertext to another - ch 22 solves this problem efficiently
                
                given a design in terms of a part library, where ach part may include other parts, list the parts in order
                so that each part appears before any part that uses it... if the design is of n parts, then there are n!
                possible orders... because the factorial func grows even faster than exponential you can not feasibly
                generate each order and that in that order each part appears before the parts using it... unless only a few
                parts were listed
                        this is a problem of topological sorting, and ch 20 shows how to solve it
                
                a doctor needs to tell if an image looks like cancer or not
                        doctor has many comprable issues of known states of cancer and not cancer
                                by using clustering (ch 33) the can identify the likely outcome
                
                file containing text needs compressed...
                        ch 15 studies huffman coding, which encodes character by bit sequences of various lengths
                                with chars occurring most frequencently encoded by shorter bit sequences
                        
                these examples have two characteristics common to many algos
                        there are many solutions to the problem many of which don't solve the problem
                        finding the 'best' without explicit examination presents a challenge

                        the have practical applications - finding the shortest path provides easy examples, a trans firm
                        such as trucking or railroads has a financial interesting in finding the shortest paths
                        through rail and road networks'

                not every problem solved by algos has an easily identified set of candidate solutions

                ex; given a set of values representing samples of a signal taken at regular time intervals the discrete
                fourier transofmr converts the time domain to the frequency domain
                        that is to say it approximates the signal as a weighted sum of sinusoudal curves... producing 
                        the strength of various freq which, which added together, approximate the sample signal...
                                fourier transform are in data compression as well as signal processing
                                also multiplying large polynomials and integers
                                ch 30 gives an algo of the fft for this problem! AWWW YEAH IMA LEARN FFTS LIKE A REAL NERD
                
                data structures are a way to store and oreganize data in order to facilitate access and modification
                        using the appropriate structure (or many) is critical for algo design
                        no single structure is best for all purposes and it's important to distinguish the differences
                        between the different kinds of structures that are most common
                
                this book will help learn how to analyze problems and use algos to our own benefit
                        many techniques are covered in the book
                
                our usual measure of efficiency is speed: how long does it take to produce an output
                        there are some though that no algo can run fast enough for, and 34 studies
                        a subset of these which are known as NP-complete
                                what makes np complete problems interesting is their potential to be solved
                                even though nobody currently has a good method for solving them
                                        essentially it is not always proven to be np-complete by fact
                        remarkeably if an efficent algo exists for any of them then efficient algos exist for all
                        this relationship among np complete probs makes solutions even more tantalizing
                        a lot of np complete problems are similar but not identical to problems that
                                we do know of efficint algos
                        cs majors are intrigued by a small change can result in big efficiency changes

                it's important to know about np-completeness and npproblems because some of them are very common
                if called upon to produce an algo for an np-complete problem you will likely waste a lot of time

                if instead, you can show the problem is np-complete, you can spend your time developing an
                efficient approximation instead

                ex; consider a delivery company with a depot... each day it loads up trucks at the depot
                the trucks get sent to deliver goods to varying addresses... at the end of the day, each truck
                must end up back at the depot so that it is ready to be loaded for the next day...
                        to reduce costs the company wants the most efficient routes to run for the lowest overall
                        distance travelled by each truck
                                * it occurs to me that this is not the best question to ask in the real world, and
                                        it'd be better to ask what is the most efficient and shortest distance traveled
                                        daily by all trucks combined - this may result in performance gains even if some
                                        trucks on some paths travel in seemingly inefficient ways
                this is referred to as the traveling salesman problem, and is np-complete...it has no known efficient algo
                        under certain assumptions though approximations can be estimated that are close to the smallest
                        possible

                alternative compute models
                        physical limitations present fundamental roadblocks to increasing clock speeds on cpus
                        since power density increases super-linearly with clock speed chips run the risk
                        of melting if running too fast
                        ... to perform faster chips are designed to contain several cores
                        ... the multiple core cpus to several sequential computers combined on a single chip
                        ... in other words parallel computing on a chip or a parallel computer
                        ... to get the best performance algos must be designed with parallels in mind
                        ... ch 26 presents models for task-parallel algos which take advantage of
                                multiple cores that run in unison
                                many modern computing platforms embrace this technique
                most ex eamples in this book assume that all of the input data is available when an algo starts
                much of the work in algos makes the same assumption...
                        for many important real-world examples this isn't the case and it arrives over time
                                the algo must decide how to proceed without knowing what data will arrive in the future
                                        in a data center jobs are always coming and going...
                                                a scheduling algo must be employed and decide when and where jobs get ran
                                                traffic must be routed from the net based on the state of the dc
                                        hospital emergency rooms make triage decisions
                                                *without knowing when other patients arrive
                                                *and without knowing when other treatments will be needed
                                                *algos that get input over time are considered _online algos_
                exercises...
                        describe a real example that requires sorting
                                cargo boxes on a container ship

                        *describe one that finds the shortest distance between two points
                                time efficiency when walking between class rooms - which hallways are the best to travel
                                or, when stocking shelves in a grocery store, which order to stock them in
                                or, even how the pallets themselves are organized when delivered to a grocery store for stocking

                        other than speed, what other measures of efficiency might be needed in the real world?
                                memory space required for calculation

                        select a data structure that I've seen, and discuss it's strengths and strengths and limits
                                arrays...
                                        can be quickly accessed by index with O(1) calculation time
                                        memory efficient being stored in contiguous memory locations
                                        predictable and simple structure with a fixed size
                                                easy to understand, reason with, and use in programming contexts
                                        easily of iteration - many algos can step through an array from top the bottom
                                        compatible with low-level operations where developers need control over memory and performance
                                        ...
                                        ...
                                        fixed size; length of an array must be specified at init which makes handling dyn data tougher
                                        slow insertion and deletion which can take O(n) time complexity making it innefficient for changing datasets
                                        contiguous memory required; arrays need to be in a block of adjacent memory locations which
                                                maay be tough to allocate depending on the scale or hardware
                                        inflexibility in terms of operations only supporting index access where other structures like
                                                linked lists, hash tables, and trees can be more flexible for tasks of faster inserts/deletes
                                                        or key searching


                        how are shortest-paths and the traveling sales-person problems similar?
                                by comparison both problems seek to solve an optimal solution
                                        shortest path tries to find the least distance between two points
                                        where as TSP seeks to solve the least between many points without repeating for cost efficiency
                        how are they different?
                                typically SP is for finding the different between two specific nodes
                                        SP has multiple potential solutions for solving in polynomial time
                                TSP is not limited to two specific nodes and applies to many nodes without repeating
                                        solving this is considered an np-hard problem with no known way of solving it in
                                        polynomial time
                                        approximations are required for the TSP problem where as for shortest path they are not

                        suggest a real world problem in which only one best solution will do
                                calculations involving money and measurement must be exactly precise in most cases
                        then come up with one that an approximate best solution is good enough
                                fast inverse square root in doom which uses multiplication to determine a vec normal
                                        for shading I think it was... famous algo hodor

                        describe a real problem in which sometimes the entire input is available before the problem needs solved
                                a dataset of images have been collected and provided - identify cats
                        but other times the input is not entirely available in advance and arrives over time
                                as tickets for a show are selling project the estimated needs for vendors to supply snacks

        C H A P T E R - 1
        ================= 1.2 - A L G O R I T H M S   A S   A   T E C H N O L O G Y
                
                if computers were infinitely fast aand memory were free, then bitcoin wouldn't be a thing
                        ... would there be a reason to study this? - yes - if for no reason than you would
                        still like to be certain your solution method terminates and does with the correct
                        answer
                
                if they were infinite fast, any correct method for solving would do - the best method
                        would likely be the easiest implementation
                                ex; randomly sort the list to get a sequence that is in the correct order
                
                computers however are not infinitely fast!
                        compute time is bounded by resource so it is precious
                                time is more valueable than money as you can not get it back
                
                Memory similarly my be inexpensive but is neither infinite nor free - algorithmic choice
                        should be limited the efficiency based on these two principles

                different algos are designed to solve similar problems often differ dramatically in their
                        efficiency.. these difference can be much more dramatic than hardware limitations impose

                as an example, ch 2 introduces two algos for sorting - firstly, insertion sort... takes roughly cn^2
                        to sort n number of items where C is a constant that doesn't depend on n - that is, it takes 
                        time about proportianal to n^2 - the second... merge sort takes roughly cn lg n where lg
                        stands for log2n nd c2 is another constant that also doesn't depend on n.... insertion sort
                        typically has a smaller constant factor than merge sort... so that c1 < c2... later it'll be
                        noted that constant factors can have far less of an impact on the run time than the dependence
                        on input size n.... we'll write insertion sorts running time a c1 * n * n and merge's as
                        c2 * n * log2(n) ... then we see that where insertion sort has a factor of n in its run time
                        merge sort has a factor of log n which is much smaller... for example when n is 1000 lg n is appro 10
                        and when n is 1m lg n is approx only 20.... although insertion sort... usually runs faster than
                        merge sort merge sorts advantage of lg n verse n more than compensates for the difference in
                        constant factors..... no matter how much smaller c1 is than c2 there is always a crossover point
                        beyond which merge sort is faster

                for a concrete example let us pit faster computer A running insert against slower computer B
                        they each must sort an array of 10m numbers - which is only about 80mb of memory so not really alot
                suppose computer A executes 10b instructions a second.... supah fast
                and computer B executes only 10m instructiors per second... supah slow
                so A is 1000 times faster than B
                to make it even more dramatic suppose the worlds craftiest dev codes insertion sort in machine lang for comp A
                and the resulting code requires 2 * n * n instructions to sort n numbers...
                suppose further that just an average dev implements merge sort using a high-level lang with an inefficient compiler
                the resulting code that's 50 * n * log(n)...
                        to sort 10 million numbers A takes
                                (2 * (10^7)^2) / (10^10) instr/sec = 20,000 seconds (more than 5 1/2 hours)
                        to sort 10 million numbers B takes
                                (50 * 10^7 * log(10^7)) / 10^7 ~ 1163 seconds, under 20 minutes
                
                by using an algo whos run time grows more slowly, even with a poor compiler, b runs 17 times faster than A
                        (unless there were less numbers potentially)
                the advantage of merge sort is even more pronounced when sorting 100 million numbers where A takes 23 days and
                B takes under 4 hours

                although 100m mght seem like a lot there are 100m web searches every half hour, and more than 100m emails sent
                every 60 seconds, and some of the smallest galaxies contain about 100 million stars... in general, as the prob
                size becomes large so does the advantage of merge sorting verses insertion sorting

                algos and other technologies...
                        the example above shows that algos like hardware should be considered like technology
                        total system perf depends on choosing efficient algos as much as on choosing fast hardware
                
                just as rapid advanced are being in other computer techn they are being made in algos as well

                are algos important on contemporary comps in light of other advanced technologies such as
                        comp architecture and fabrication tech
                        intuitive guis
                        object oriented systems
                        integrated web techs
                        fast networking wired and wireless
                        machine learning / ai
                        and mobile
                                (yes stupid)
                
                although some apps do not explicitly require algo content at the app level many do

                ex; web services that determine travel distance... the impl would need fast hardware
                        a gui
                                wide networking
                                        object orientation possibly
                                                ... and also badass algos for ops such as route finding
                                                        rendering maps
                                                                interpolating addr's
                
                moreover apps that don't require algo content at app level still rely on them
                        because one thing depends on the other, and many things use algos, the naswer is generally yes
                
                machine learning can be thought of as a method for performing algo tasks without explicitly designing an algo
                but instead inferring patterns from data, and therby auto learning a solution

                at first glance, ML, whcih automates, may seem to make algo learning obsolete - that is false

                ML itself is a collection of algos just under a different name

                it currently seems that the success of ML are mainly problems for which we as humans don't really
                        understand what the right algo is
                
                prominent examples include computer vision and auto language translation
                
                for algo probs that humans know well such as most in this book efficient algos designed to solve these problems
                are typicalling more successful than ML approaches

                DS is an interdisciplinary field... with the goal of extracting brainz and hax from structures and unstruc-
                tured data... data science uses statistics, CS, and optimization...
                        the diagnosing of algos is fundamental to this field

                the core techniques of DS which overlap with those in ML include many in this book

                with bigger computahs we use em' to solve larger problems as better machines are developed

                as seen in the above comparison between two sorting algos it is at large problem sizes that the
                differences in efficiency 

                having a solid base of algo knowledge and technique is on characteristic that defines a truly skilled dev
                
                modern tech you can accomplish some tasks without knowing much about algos, but with a backgorund in algo
                you can do so much more ((:)))

                exercise...

                        1.2-1
                                give an example of an app that requires algo content at the app level, and discuss the func of the algos
                                        my ratatui app in rust uses the cassowary algo to break a rectangle down into smaller rectangles
                                        quickly and efficienctly to determine the placing of rectangles in a terminal layout
                                
                        1.2-2
                                suppose that for inputs of size n on one machine, insertion sort runs 8n^2 steps
                                and merge sort runs 64 * n * log(n) - for which values of n does insert sort beat
                                merge sort...
                                        values of n that would beat merge sort with insertion sort would be values of
                                        n in which the formula 8n^2 resulting in an answer less than the value of the
                                        result returned by 64 * n * log(n)... more specifically values of n lower than...
                                ex;
                                        8 * n * n < 64 * n * log(n)
                                                divide both sides by 8
                                        n * n < 8 * n * log(n)
                                                divide both sides by n (for n > 0)
                                        n < 8 * log(n)
                                                approximate by testing values of n
                                        30 < 8 * log(30) ~ 8 * (3.401) = 27.208 = FALSE
                                                so in this instance values of about 30 or less to sort are faster with insert sort
                                                but values of n greater than 30 would be faster with merge sort
                        
                        1.2-3
                                what is the smallest value of n such that a run time of 100n^2 is faster than those running time is 2^n

                                        100*n^2  <  2^n
                                        100*1^2  <  2^1
                                                100*1  <  2 FALSE
                                        100*5^2  <  2^5
                                                500^2  <  32 FALSE
                                        100*10^2  <  2^10
                                                100 * 100 < 1024
                                                        10000 < 1024 FALSE, but CLOSER
                                        100*15^2  <  2^15
                                                22500 < 32768... TRUE!...
                                        100*14^2  < 2^14
                                                19600 < 16385... FALSE

                                        for values of n that are less than 15 2^n is faster, but very quickly becomes much MUCH slower
                                                I believe this would be 2!, and is the slowest possible that I've learned
                                                        2 factorial!

                        for each function f(n) and time t in the following table... determine the largest size n of a problem that can be solved
                        in time t... assuming that the algo to solve it takes f(n) microseconds

                                asked to do a bunch of math to fill out a table, but I just did the highs and lows to see the excessive differences

                        using a gpt I got estimates for 1 sec, 1 min, 1 hour, 1 day, 1 month, 1 year, and 1 century estimated in microseconds

                        where 1 sec is 1,000,000 microseconds 1 century is 3.15x10^15 micro... or ~3.15 quadrillion microseconds
                                a year being 365.25 days

                        log(n)  ... 1 second, 2^(10^6)...       1 century, 2^(3.15x10^15)       ... GYATT DANG
                        sqrt(n) ... 1 second, ~10^12...         1 century, 9.9225(10^30)        ... wew!
                        n       ... 1 second, ~10^6...          1 century, 3.15x10^15           ... still many!
                        n*log(n)... 1 second, ~200,000          1 century, ~2.2x10^12           ... 2.2 trillion runs ...ish
                        n^2     ... 1 second, 1000...           1 century, 5.6x10^7             ... 36.25 million actually whoops
                        n^3     ... 1 second, 100..!            1 century, 146,500              ... oof! 3.6m actually?
                        2^n     ... 1 second, 20.. :(           1 century, 51.46
                        n!      ... 1 second, 9-10              1 century, 16-17                ... boiii I need to redo this whiteboard math, but the idea is clear
                                                                                        with n! clearly be even worse than exponentially slower to solve over
                                                                                                a duration of time it seems
                                                                                                        2^n being exponential wat is factorial called then
                                                                                                                factorialy? slower?
                C H A P T E R 2   -   G E T T I N G   S T A R T E D

                this chapter introduces the framework that's used throughout the material to get familiar with algo design
                starting with insertion sort to solve the sorting problem introduced in chapter 1
                algos will be specified with pseudocode
                it will be shown why, or rather how insertion sort sorts things and analyze it's run time
                the analysis will introduce a notaiton that describes run=time increases with the number of items sorted
                following insertion sort, will be merge sort and the chapter will end by analyzing it's run time as well

                ================= 2.1   I N S E R T I O N   S O R T ======================================================================================================

                the first algo solves the sorting problem in ch 1
                        input: sequence (a1, a2,....,an)
                                output: permutation (a'1, a'2,....,a'n) of input such that a1' <= a'2 <= ... <= a'n

                the numbers to be sorted are known as keys

                although the problem is conceptually about sorting a sequence, the input comes in the form of an array of a arbitrary element count n
                        when sorting is wanted often it's because the keys refer to data points which is referred as satallite data
                                combined keys and satellite data form records
                ex; consider a spreadsheet containing records with many pieces of data - like age, GPA, course count, name, etc
                        any of satellite data could be considered a key, but when sorting, it moves the associated satallite data with the key
                when describing a sort algo the focus is on the keys, but it's important to remember that there usually is associated data
                typically when describing algos as procedures they're written in pseduocode that is similar in many respects to c, c++, c++, java, python, or javascript
                        if any of these langs or many others are known then pseudocode should be legible generally speaking
                                the difference is that whatever method is most concise for explanation is used given any particular algo
                                        it's like code english

                pseudo code often ignores aspects of software engi. such as abstraction modularity and error handling in order to convey an algo concisely

                we start with insertion sort
                        which is efficient algo for sorting a small number of elements
                                it works how you might sort a hand of cards
                                        start with an empty left and cards in a pile on the table
                                                pick up the first card and put it in the left hand
                                                        then with the right hand, remove one card from the pile at a time
                                                                insert the card into the correct position in your left hand
                                                                        if the card to insert is greater than to the left most card
                                                                                move to the right, and check again
                                                                                        and check again
                                                                                                until it isn't greater than the card be looked at
                                                                                                        insert

                at all times the cards in the left hand are sorted
                        these cards were originally the top most cards in the pile on the table

                the pc for insert sort is given as the procedure I N S E R T I O N - S O R T on the facing page...
                        it takes two params
                                array A containing values to be sorted
                                        and 'n' number of values to sort
                                                the vals occupy positions A1 ... An in the array
                                                        which is also denoted by A[1:n]...
                                                                when the procedure is finished A[1:n] contains the original values, but in order

                

                a figure is shown for illustration with some monkey paws
                        you find the correct position by comparing it to the cards already present
                
                first pc (pseduo-code) example

                INSERTION-SORT(a,n)                             <--- name of function, and params
                        for i = 2 to n                          <--- start at 2, iterate through n number of items?
                                key = a[i]                      <--- get the key at index i in the array
                                j = i - 1                       <--- set a j value to 1 less than i
                                while j > 0 and a[j] > key      <--- while j is greate than 0 (so not the first iteration really) and a[j] > a[i] execute the next calls
                                        a[j + 1] = a[j]         <--- set the key adjacent to a[j] by +1 to a[j]
                                        j = j - 1               <--- subtract 1 from j
                                a[j + 1] = key                  <--- after shifting loop, set the right-adjacent key to j to the original key at a[i]
                
                .... loop invariants and correctness of insertion sort
                
                a figure shows how this algorith works for an array a that sorts out with [5, 2, 4, 6, 1, 3]
                        index i indicates the 'current card' being inserted into the hand
                                at the start of each iter of the for loop (indexed by i), the subarray consisting of a[1:i-1]...
                                        constitutes the currently sorted hand
                                                the remaining subarray being a[i+1-n]
                                                        corresponds to the pile of cards on the table still
                                                                in fact, elements a[1:i-1] are are the elements in positions 1 through i - 1, but now sorted
                                                                        we state them a s properties of a[1:-1] formally as a loop invariant
                                                                                meaning this part of the iteration is not changing I think
                                                                        yes but I didn't understand quite right, it's the input state before the algo operates
                                                                        each loop invariant is the state again after the iter
                                                                        so the diagrams shown in the book are each 'invariant' as the loop begins, and
                                                                                then the diags show the iterative changes for that loop describing the changes
                                                                                        before the next invariants
                
                figure descriptions...
                        given the array before [5 2 4 6 whatever] 
                                starting at i = 2 in the for loop
                                        key is set a a[i], key = 2
                                        j is stated and becomes 2 - 1, so 1, j = 1
                                                while j > 0, (1 > 0) AND a[j] > key (5 > 2)
                                                        a[j + 1] = a[j], the value at j+1 in a is now = to a[j], so a[2] = a[1] now, 5 becomes 2 in the array
                                                        j = j - 1, or 1 - 1, j = 0!, j is no longer greater than 0, while loop completes
                                                a[j(0) + 1], a[1] becomes the key, which was 2, so a[2] = 5 now
                                        // next iteration happens
                                        key is now 3, j = 2, iters will stop after first because a[j] < key after first iter (4 < 2 is false)
                                        // next iter, nothing appens, 6 is not < 5
                                        // many iters happen in the while loop, 1, swaps all the way down to the first index, indexes after first shift right until key
                                        // 3 midway shifts, and now the list is sorted out
                
                loop invariants help to understand why algos are correct.... when using loop invariants three things get shown
                        initialization...
                        maintenance...
                        termination...
                                ...the the at init is true for the iteration of the loop - as in the invariants start is in fact correct
                                ...if it is true before the iteration, it remains true before the next iteration
                                ...the loop terminates, and when it does, the invariant (along with a reason why the loop terminated)
                                        gives a useful property that helps show the algo is correct
                
                when the first two properties hold, the loop invariant is true prior to every iter of each loop
                        other facts beyond the variant itself can be established... to prove the invariant remains true before the iteration
                                a loop invariant proof is a form of mathematical induction where it's proved a property holds
                                        a base case is proved, and then an inductive step is proved
                                                here, showing an invariant holds before the first iter corresponds to the base case
                                                        also showing showing that the invariant holds from iter to iter corresponds(?) to the inductive step
                
                the third property is the most important since the loop invariant is used to show correctness...
                        typically the loop invariant along with the condition that caused the loop to terminate...
                                mathematical induction typically applies the inductive step infinitely, but in a loop invariant the induction stops
                                        ... when the loop terminates
                
                below details these three properties with the example of insertion sort provided
                        initialization...
                                start by showing the loop invariant holds before the first iteration (when i = 2)
                                        the subarray a[1:i-1] consists of just the single element a[1]
                                                which is in fact the original element a[1]
                                                        more: this subarray is sorted considering how could it not be with just 1 val
                                                                this shows the loop invariant holds prior to the first iter of the loop!
                        maintenance...
                                next, the second prop, showing eah iter maintains the loop invariant...
                                        the body of the for loop works by moving the values in a[i-1], 2, 3 and so on by one position to the right...
                                                until it finds the proper position for a[i] at which point it inserts a[i]...
                                                        the subarray a[1:i] then consists of the elements originally in 1-i but in sorted order
                                                                i increments increasing the val of i by 1 for the next iteration of the for loop them preserving..
                                                                        the loop invariant
                        
                                formally, treating the second property would rquire us to state and show a loop invariant for the while loop of lines... 5-7
                                        ... since there is a loop inside of this loop, but we're skipping that to not get bogged down with invariants
                                                ...quite yet... instead relying on informal analysis to show the second property holds for the outer loop

                        temrination...
                                finally we examine loop termination...
                                        the loop var i starts at 2 and increases by 1 in each iter...
                                                once i exceeds n - in line 1, the loop terminates...
                                                        so the loop terminates once i equals n + 1
                                                                substituting n + 1 for i in the wording of the invariant shows that a[1:n] contains the original elements...
                                                                        of a[1:n], but in sorted order
                                                                                hence, the algo is correct
                        
                        this method of loop invariants is used to show correctness in various places in the book

                        ps (pseudocode) conventions
                                we use the following conventions in the code
                                        - indentation indicates block structure or scope which is typical for any languages
                                        - the looping constructs (while, for  repeat until, and if else) have interps similar to those in ther languages
                                        - // is for comments
                                        - vatriables like i j key are local to the given procedure - there aren't globals unless explicitly indicated
                                        - array access by specifying the array name following by the index in brackets...
                                                ex; a[i] indicates the i'th element of array a
                                                        many langs enforce 0-origin - they choose wwhatever is clearest for human readers because counting starts at 1 ordinarilly
                                                                bounds will be expressly indicated if 1 or 0 in invariant diagrams and such

                notes taken directly... not fixing LaTeX encoding crap... I can read it fine
                3 In an if-else statement, we indent else at the same level as its matching if. The ûrst executable line
                        of an else clause appears on the same line as the keyword else. For multiway tests, we use elseif for
                        tests after the ûrst one. When it is the ûrst line in an else clause, an if statement appears on the line
                        following else so that you do not misconstrue it as elseif.
                4 Each pseudocode procedure in this book appears on one page so that you do not need to discern
                        levels of indentation in pseudocode that is split across pages.
                5 Most block-structured languages have equivalent constructs, though the exact syntax may differ.
                        Python lacks repeat-until loops, and its for loops operate differently from the for loops in this book.
                        Think of the pseudocode line <for i D 1 to n= as equivalent to <for i in range(1, n+1)= in Python.
                6 In Python, the loop counter retains its value after the loop is exited, but the value it retains is the
                        value it had during the ûnal iteration of the for loop, rather than the value that exceeded the loop
                        bound. That is because a Python for loop iterates through a list, which may contain nonnumeric
                        values.
                        
                                next page... if the language doesn't use 0 as the starting index, or does, or whatever the case....
                                        then give yourself credit for being able to just adjust
                                                can either always subtract 1 from each index, or allocate each array with an extra position and ignore 0

                                        -compound data is organized into objects
                                                these objects are composed of attributes
                                                        the syntax for access attributes of an object will be . syntax, like object.attrbute
                                                                each var representative of an object is treated as a pointer or reference to the data
                                                                        for all attr _f_ of an object _x_, setting _y_ = _x_ causes _y.f_ to equal _x.f_
                                                                                more; if we not set x.f = 3, then afterward not only does x.f = 3, so does y.f
                                                                                        rexplained, x and y point to the same reference to an object attr f
                                                                                                this is a way of treating arr and obj consistently with most languages
                                                our attr notation can cascade, for ex; suppose that the attr  f is itself a pointer to some type of obj with attr g
                                                        then the notation x.f.g implicitly parenthesized as (x.f).g, rehashed, if we had assigned y = x.f...
                                                                then x.f.g is the same as y.g..!
                                                sometimes a pointer refers to no object and in this case we give it the special val of NIL
                                        -params are passed to a procedure by value: the called function receives its own copy of the params...
                                                if the procedure assigns a value to a param, the change IS NOT SEEN by the calling procedure... scoped local change...
                                                        when objects are passed the pointer to the data of the object is copied... but the attr's are not...
                                                                ex; if x is a param of procedure then x = y within procedure is not visible to the procedure
                                                                        the assignment of x.f = 3 is visible if the procedure as a pointer to the same object as x
                                                                                similarly arrays are passed by pointer as opposed to the entire array... like rust refs...
                                                                                        and changes to the array elements are visible... most contemp languages
                                                                                                do not work this way (???????)
                                        -return statement immediately transfer control back to the point of call in calling procedure
                                                most return statements also take a val to pass back to the caller
                                                        PS differes from many langs in that we allow multiple vals to be returned in a single return without
                                                                having to create objects to package them together
                                                                        ... in rust this is like returning a tuple!
                                        -bool operators AND and OR short circuit...
                                                that is to say when evaluating x AND y, x is evaluated first... if x is FALSE then the entire expression
                                                        cannot evaluate to true, and therefore y is not evaled
                                                on the other hand if x evals to true, y must eval to determine the val of the expression
                                                        seems sequential...
                                                                similarly in the expr x OR y the expr y is only evaled if x evals to FALSE...
                                                                        short-circuiting ops let us write expressions like x != nil AND x.f = y...
                                                                                without worrying about what happens upon evaluating x.f when x is NIL
                                        
                E X E R C I S E S !

                        2.1-1 using fig 2.2 as the model illustrate the operation of insert sort on an array containing 31 41 59 26 41 58... so perhaps in rust?...
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
                                gpt analysis:
                                        two binary numbers a, and b
                                                - a[0:n-1] represents binary number a, in little endian format (least significant at 0)
                                                - b[0:n-1]: in the same format
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
                                                        overflow or carrying
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


                ================= 2.2   A N A L Y Z I N G   A L G O R I T H M S

                disecting and analyzing algos has come to mean predicting the needed resources for the algo to execute successfully
                        the resources such as bandwidth, energy usage, memory consumption may need to be considered
                                more often than not you'll want to measure the computational time however
                
                before taking figuring out an algo you need a model of the technology that it runs on
                        in addition to that a way to express the cost of the operation
                                most of the book assume a generic single core RAM model of computing understanding algos are
                                        implemented as computer programs.... RAM model instructions execute on after another
                                                with no concurrent operations
                                                        the ram model assumes that each instruction takes the same amount
                                                                of time as any other instruction and that each access to data using a var
                                                                        or staring into a var takes the same amount of time as any other data access
                                                                                ... in other words the in the ram model each instr or data access..
                                                                                        takes a constant amount of time - even array indexing
                                to summarize the above, all operations take the same amount of time to do even though that isn't true
                                        we're just going to make that assumption for the simplicity of calculation
                instructions should be precisely defined in the ram model, and their costs =
                        to do this would be tedious and yield little insight into algo design an analysis
                                yet we must be careful to not abuse the ram model....
                                        ex; what if a RAM had a sort instruction - then you could sort in just one step!
                                                such a model would be unrealistic because this kind of instruction doesn't exist on a real computer
                                                        our guide will be _real computer- design... instructions for the ram model are those
                                                                commonly found in computers... add, sub, multi, div, %, floor, ceil etc
                                                                        ...load store copy control subroutine call and return
                the data types in the ram model are ints floats and char
                        real computers don't usually have a seperate type for bools
                                instead they often test an integer for 0 or not 0 for false or true
                                        typically we don't worry about float precision in this book although it is crucial in most applications
                                                also assume each word of data (00010011) has a limit on bit count...
                                                        for ex when working with inputs of size n assume ints are represented by C log2(n) for some constant
                                                                of c that is >= 1 where c >= 1 is required so that each word can hold the value of n
                                                                        which enables us to index the individual input elements
                                                                                additionally c is restricted to being a constant so that the word can not
                                                                                        arbitrarily change in size... if it did could store a lot of data in one...
                                                                                                word and operate in constant time which is unrealistic
                real computers contain instructions not listed above - and those instructions will be considered a gray are in the RAM model we're using
                        ex; exponentiation is not a constant time op takes time of log(n) and you must worry about the result fitting into a word...
                                if n is an exact power of 2 expoenetiation can be typically seen as as a constant time op since bit shifting ops exist
                                        in most computers shifting is the same as multiplying or dividing by 2 - so such computers computer 2^n in 1
                                                constant-time instr by shifting.... we'll be avoiding those kind of gray areas in the RAM model
                                                        and treat calcs for 2^n, and multiplying 2^n as constant-time operations when the result...
                                                                is small enouhg to fit into a single word
                the RAM model doesn't account for typical memory hierarchy in a computer.... it does not model cache, or virtual memory...
                        many comp models attempt to account for this, which are sometimes significant in real programs on real computers...
                                s11.5 and a few other examples get into this but overall this isn't going to be analyzed very much...
                                        models that include memeory hier. are more complicated  and are more difficult to work with than the scope of this book
                                                realy gets into... outside of this the RAM model analyses are pretty good predictors for perf on real comuters
                although it is often straight forward to analyze an algo in this mode sometimes it's challenging
                        you may need to employ math tools such as combinators, probability theory, algebraic dexterity, and indentifying most significant terms
                                since algos can behave differently for each input we need a means for summarizing these behaviors easily
                anaylsis of insertion-sort
                        how long does the procedure take? one way to tell would be to run it and look at the run time, so of course it'd have to be implemented
                                in reality for this to happen since the pseudocode couldn't be run directly... what would this kind of test reveal? you'd find
                                        how long it took for insert-sort to run on the computer it ran on a particular input provided under a specific implementation
                                                with a specific compiler/ interpreter.. with specific libraries... and while the computer was running other tasks
                                                        concurrently with the execution of the insert-sort that was ran...
                                                                even running the same thing on the same machien the results may be different
                                                                        let alone running it on another machine... realisticially not much can be consisistently determined
                                                                                from one setup to anohter.... we need a way of predicting given any input how long insertion will take
                instead of timing a run of insertion sort this way we can figure how long it would take to calc for by analyzing the algo itself... this book will examine how
                        many times it executes each line os ps and how long each line of ps will take to run... we'll come up a precise but complex formula to figure for the run time
                                ... then identify the most important parts of the algo using a convenient notation that can help us compare the run times of different algos for the
                                        same problem
                first, lets note that the run time depends on the input - it shouldn't be a surprise that sorting thousands takes longer than
                        ... sorting three numbers... further insert sort can take different amounts of time t osort two arrays of the same size depending on their starting
                                ... sortedness - focusing on the one that has the greatest effect - the input size - we'll describe the running time of a prog as a f() of the size
                                        ... of it's input.... to do this we'll have to outline that terms running time and input size more precisely... we need to be clear on whether
                                                ... we're indiciating running time for an input that elicits a worst case behavior, and a best case behavior, or some other case of note
                the best idae for input size is dependent of the problem at hand - for a lot of issues such as sort, or fourier transforms - the most nat measure is the num of items
                        that are considered in the input.... ex; the number n of items to sort - for other problems such as multiplication the best measure is the number of bits needed
                                to represent some input that's being multiplied... ex; if the input of an algo is a graph we'd usually characterize the input 
                                        by both the number of verticies, and the number of edges or sides to the graph... we'll indicate which
                                                input size measure is being used with each problem as we get to them
                the running time of an algo on some input is the number of instructures and data access up for execution... how we account for these costs
                        should be independent of a particular computer, but within the framework of the RAM model - we'll adopt the following view for now...
                                a const time is required to execute every line of ps.. one line might take more or less than any other line, but we'll assume that each exec of
                                        the kth line takes Ck time where Ck is a constant....
                                aa constant time is required for executing each line of ps... a single like may take more or less tha nanother, but we're assuming
                                        each execution of the Kth line takes Ck time where Ck is a constant - eg line K executes in Ck amount of time since Ck is cconstant...
                                                this view is in keeps with RAM model, and reflects how the PS would work on most real computers
                considering insert-sort; as noted above... we start by devising a formula that uses the input size, and all statement costs Ck...
                        this formula turns out to be a bit messy....
                                we;ll then switch to a simple notation that is more concise and easier to use
                                        this simpler notation makes clear when comparing run times of algos especially as input size changes
                to analyze insert-sort - well view it on the next page with the time cost of each statement, and the number of times each statement executes..
                        for each i = 2, 3, 4, ... n; let Ti denote the num of times the while loop test in line 5 is executed for that value of i....
                                when a for or while loop exits normally - because the test in the loop head comes up as false - the test is exec'd one time more than the loop body
                                        because comments are not exec'd statesments assume they take no time
                the run time of the algo is the sum of run times for each statement exec'd; a statement that takes Ck steps to exec and executes m times contributes CkM to the total time
                        or steps * execution per step.... we usually denote the running of an input of size n by T(n)....
                                to compite T(n), the run time of insert-sort, on an input of n vals we sum the products of the cost and times columns.. obtaining.

                INSERTION-SORT(A, n)                    cost    times
                for i = 2 to n                          c1      n
                        key = a[i]                      c2      n-1 (one times less the the total tests against i=2 to n)
                        //insert a[i] into sort bla     0       n-1
                        j = i - 1                       c4      n-1
                        while j > 0 and a[j] > key      c5      SUM(i=2..n) Ti
                                a[j + 1] = a[j]         c6      SUM(i=2..n) (Ti - 1)
                                j = j - i               c7      SUM(i=2..n) (Ti - 1)
                         a[j + 1] = key                 c8      n-1
                
                T(n) = c1 + c2 + c4 + c5 + c6 + c7 + c8
                T(n) = n + (n - 1) + (n - 1) + SUM(i=2..n) Ti + SUM(i=2..n) (Ti - 1) + SUM(i=2..n) (Ti - 1) + (n - 1)

                even for inputs of a given size and algo's running time may depend on which input of that size is given...
                        ex; in this algo the best case occurs when the array is already sorted... in this case each time line 5 executes, the val of key in a[i] is already
                                ...greater than or equal to all values of a[j] or A[1:i-1] - so the while loop always exits on the first test line...
                                        therefor we have that ti = 1 for i = 2, 3, ..., n and the best case run time is given by

                                        (c1, c2, c4, c5, c8)n - (c2 + c4 + c5 + c8) 

                *** WARNING MUCH SKIPPED HERE... TOOK TO WHITEBOARD TO DO SIMPLIFICICATION STEPS AND STUDY THIS ALGO UNTIL I GOT IT
                        *** THIS STOPS MAKING SENSE HERE FOR A MOMENT AND JUMPS BACK ON AT THE END OF THE EXPLANATION FOR THIS FIGURE
                
                typically as insert sort, the run time of an algo is fixed for input n, although later randomization of the input will be
                        explored...
                worst-case and average-case analysis
                        our analysis of insertion sort looks at both the best and worst cases of the algo in which an input was in reverse order
                                and also in sorted order to start....
                                        for the remainder of the book though we'll usually but not always go for only worst-case run times -
                                                that is, the longest possible for any input of size n - with reasons as follows
                                                        worst case running time gives a highest bound on running time for ANY input- we're guarenteed
                                                                by our assumption to be within this time - no guessing required if it may be worse
                                                                        this is especially important for real time computation
                                                        for some algos... the worst is often the case ex; in search a db the search algo worst case often
                                                                occurs when the info isn't in the db... in some applications searches for no absent info
                                                                        could frequently be the case
                                                        the average case is roughly asa bad as the worst... suppose that you run insert-sort on some array of
                                                                randomly chosen numbers... how long would it take to determine where subarray a[1:i-1] to insert
                                                                        element a[i]? on average, half the elements are less than and half are greater on average
                                                                                therefore a[i] is compared with just half the subarray... so ti is i/2 - the result
                                                                                        average-case running time would turn out to be a quadratic f(n) just like
                                                                                                the worst case running time
                in some cases we'll be interested in the average case run time of an alog - we'll see probabalistic analysis applied to various algos in the book
                        the scope of average case analysis is limmmiittted...! because it might not be obvious what is average input for a specific problem
                                often we'll just figure that all vars of some n are equally likely... in practice this may be wrong, but we can use randomized
                                        algo which makes random choices to allow probabilitistc analysis and yield expected running times - we'll explore rando
                                                algos more in ch 5 and in a few other chapters subsequently
                O R D E R O F G R O W T H
                        to make ease of analyzing insert-sort procedure, we had used some abstractions of simplification - first, we ignored the cost each statement
                                using the constants Ck to rep these costs - still the best and worst case run times in 2.1 and 2.2 are rather unwieldy... the c's
                                        in these gave us a lot of detail when it wasn't required.... that's why we also expressed the best case run time as
                                                an + b, for consts a and b on the statement costs k, and similarly an^2 + bn + c for constants a b and c that 
                                                        depended on the statement cost Ck.... we thus ignored not only the actual cost, but also the abstract
                                                                cost of Ck
                lets make another more simplfying abstraction; it is the RATE OF GROWTH or ORDER OF GROWTHHHH of the running time that really interest us
                        ... we therefore consider only the first term of a formula (eg, an^2) since the lower order terms are insignificant in comparison
                                for large values of n... we also ignore the leading term's constant coefficient - since constants are less significant in the rate
                                        of growth... in determining computatational efficiency for large inputs.... for insert-sort worst case run time
                                                when ignoring the lower order terms and the leading term's constant coefficient, only the factor of n^2 remains
                                                        that factor of n^2 is BY FAR the most important part of the running time - for example... suppose that
                                                                an algo implemented on a particular machine takes n^2/100 + 100m + 17 micro on an input of n
                                                                        althoug hthe coef's of 1/100 for n^2 term and 100 for the n term differe by four orders
                                                                                of magnitude the n^2/100 term D O M I N A T E S 100n one n exceeds 10k... 10k
                                                                                        may seem large - it is smaller then the average town population...
                                                                                                many real world problems have much larger input sizes
                to highlight the order of growth of the run time we have a special notation that uses greek O theta. we write that insert sort has a worst case run time of thetaO
                        (n^2).... pronounced theta of n-squared or just theta n squared... we write that insert sort has a best caase of theta of n or theta n... for now think of 
                                theta notation as saying 'roughly proportional when n is large' so that theta n ^2 means ruoughly proportional to n^2 when n is large
                                        and theta(n) means roughyl proportiona lto n when n is large.... we'll use theta-notation informally in this chapter and define it precisely
                                                in ch 3...
                we consider on algo to be more efficient than another if it's case run time has a lower order of growth... ! due to constant factors and lower order terms
                        and algo whos run time has higher order of growth might take less time for small inputs than an algo whose run time has a lower order
                                BUT n larger inputs.... an alo whose worst case run time is theta N^2 for ex takes less time in the worst case than an algo whose
                                        worst case run time is theta of n cubed... regardless of the consants hidden by theta notiation there is always some
                                                number - say n0, such that for all input sizes n >= n0 - the theta n squared algo beats the theta n cubed algo in the worst case
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
                        
                        consider linear search again... how many elements  of the input array need to be checked on average... assuming that the element is equally possibly in
                                any index in the array... and additionally how baout the worst case
                        
                        in the average case O(n/2) since approximately half would be looked through on average, and in the worst case O(n) since the search index either isn't
                                present or is in the last index of the array
                                                        CLAUDE: EXACTLY!
                        
                        how can any sorting algo be modified to have a good best-case run time?
                                check if it's sorted ahead of sorting
                                                        CLAUDE: GREAT! yes, that is a good idea
                        
                2.3 - D E S I G N I N G   A L G O R I T H M S
                you can choose from many algo design techniques - insertion sort is considered the incremental method
                        ... for each element A[i] insert it into it's proper place in the subarray a[0..i-1]
                this section examines another method known as 'divide and conq' DaC for short which will be explain even more in chapter 4...
                        we'll use this method to desig na sort algo with a worst case run time is much better than insertion sort's by comparison
                                one advantage of using an algo that follows DaC is analyzing it's run time is often straight forward..
                                        using techniques we'll explore in ch 4
                2.3.1 the dac method
                many useful algos are recursive in structure: to solve a prolem they call themselves one or more times to handle closely
                        ...related sublems... these algos typically follow the daq method; they break the problem into several subproblems that are si-
                                ...milar to the original problem but smaller in size - solve the subproblems recursively, then combine the results
                                        ... to create a solution to the original problem
                in the daq method if the problem is small enough - the base case - you just solve it without recursion... else the recursive case
                        there are three steps...
                                div: the problem into one or more subproblems that are smaller versions of the same problem
                                conq: solve the subproblems recursively
                                combine: combine the solutions to form a solution to the original problem
                mege sort follows the dac method
                        in each step it sorts a[p:r] starting with the entire array a[1:n] and recurses down to smaller and smaller subarrays
                        here is how merge sort operates...
                                div: divide subarray a[p:r] to be sorted into two arrays - each half of it's size
                                        compute the midpoint Q of a[p:r] taking the average of p and r and divide into a[p:q] and a[q+1:r]
                                conq: sort the two subarrays a[p:q] and a[q+1:r] recursively using merge sort
                                combine: merge the two sorted subarrays ... a and b back into a[p:r]
                the recurse bottoms out when the base case a[p:r] has only a single element; p is equal to r; as noted in sert-sort's loop
                        ..invariant an array of only a single index is always sorted
                                the key op of the merge sort algo occurs in the combination step which merges to adjacent sorted subarrays
                                        .... the merge op is performed by the aix procedure MERGE(a, p, q, r) on the following page...
                                                ... where A is the array and p q r are indexes into the arr such that p <= q <= r....
                                                        the procedure assumes that the adj subarrs a and b were already sorted - 
                                                                it merges the two sorted subarrs to form a single sorted subarr that replaces subarr...
                                                                        .... a[p:r]
                the understand how merge proc works lets go back to the cards...
                        suppose you have two piles face up
                                ... each pile is sorted with smallest values on top
                                        you want ot merge the two piles into a single pile which will be face down
                                                the basic step consists of choosing the smaller of the two on top of the face up piles...
                                                        ...removing from it's pile when exposes a new top card and placing it face down
                                                                repeat this until one input pile is empty
                                                                        at which time you just take the remaining pile and flip the entire pile
                                                                                placing it face down on the output pile
                how long would this take? each step takes constant time - since comparing just 2 cards - each

                //not going over the verbose description of merge-sort; instead I just wrote the functions in rust for a pure understanding of the algo
                //instead of depending on reading comprehension to kind of get it

                2.3.2   A N A L Y Z I NG   D I V I D E - A N D - C O N Q U E R   A L G O R I T H M S

                        when algos contain recursing call their run times can be described by a recurrence equation or recurrence... which often
                                describes the overall running time on a problem of size n in terms of the running time of the same algorithm on smaller
                                        inputs..! mathematical tools can be used to solve the recurrence and provide bounds on the performance of the algo
                
                a recurrence for the run time of a dac algo falls out from the three steps of the basic method
                        as we did for insertion sort let T(n) be the worst case run time on a problem of n size.... if the problem size is small enough
                                say n < n0 for some constant n0 < 0... the straight forward soluton takes constant time... which we write as O(1).... suppose
                                        that the division of the problem yields A subproblems.... each with size n/B, that is, 1/B - the size of the original -
                                                for merge sort both a and b are 2, but we'll see ohter dac's in which they aren't equal - it takes T(n/b) time to solve one
                                                        subproblem of n/b... and so it'd take aT(n/b) time to solve all A of the problems... if it'd take D(n) time to divide
                                                                the problem into subproblems... and C(n) time to combine the solutions to the sub problems into one solution
                                                                        to the originial problem... we get the recurrence... a figure is shown visualizing the dac method
                                                                                in merge-sort
                                                                                        ... dividing a length of 8 3 times into sub problems of 2, then 4, then 8 all of 1 item vectors...
                                                                                                then immediately following division is the merging step in equal quantities
                                                                        figure 2.4 the op of merge sort on A with length 8 that contains a sequence - p, q, r all appear in each subarray
                                                                                above their values... numbers in italics indicate the order in which m-s and m proc's are called following
                                                                                        the initial call of merge-sort

                        a fomula is shown for the calculation of merge sort
                                T(n) = O(1) if n< n0, otherwise D(n) + aT(n/b) + C(n)
                
                ch 4 will show how to solve recurrences of this form

                sometimes the n/b size of the div step isn't an int - for ex; m-s proc divs problem of size n into subprobs of sizes n/2 and n/2, since the diff between them is
                        at most 1... which for large n is much smaller than the effect of div n by 2 - we'll adjust and say they are both n/2 because it hardly matters in this context
                                as ch 4 will discuss; this simplification of ignoring floors and ceils does not generally affect the OoG of a solution to dac recurrence
                another convention we'll adopt is to omit a statment of base cases of recurrence... which we'll also get into in ch 4... the reason is that base cases are almost always
                        T(n) = O(1) if n < n0... for some constant n0 > 0 - that's because the run time of an algo on an input of constant size is constant; we save lots of extra writing
                                by adopting this convention to save time!
                analyzing merge sort
                        here's how to set up the recurrence for T(n); the worst case run time is merge sort on n numbers...
                                Div: the div step computers the middle of the sub, which is in constant time therefore D(n) = O(1)
                                Conq: recursively solvin two subprobs, each of size n/2, contributes 2T(n/2) ignoring ceils and flooors
                                Comb: since th merge proc on n-element sub-arrays takes O(n) time we have c(n) = O(n)
                when adding the funcs D(n) and C(n) for the merge sor tanalysis, we are adding a function that's O(n) and a function that is O(1) - this sum is a linear func of n...
                        that is it is roughly... proportional to n when n is large, and so merge sort's dividing and comining times together are O(n)... add O(n) to 2T(n/2) term from the conq
                                gives a recurrence for the worse case run time T(n) of merge sort
                                        T(n) = 2T(n/2) + O(n)
                ch 4. presents the master theorem - which shows that T(n) = O(n lg n)
                        compared with insertion sort whose worst case run time is O(n^2) merge sort trades away a factor of n for a factor of lg n... because the log func grows more slowly
                                any linear func that's a good trade.... for large enough inputs... merge sort with it's O(n lg n) worst case running time, outperforms insertion sort, whose
                                        worst-case running time is O(n^2)
                master theorum isn't required to understand why the solution to recurrence 2.3 is T(n) = O(n lg n); for simplicity assume n is an exact power of 2, and that the implicit base
                        case is n = 1... then recurrence (2.3) is essentially... constant time if n = 1, or 2T(n/2) + cn if n < 1
                where c1 > 0 represents the time required to solve a problem of size 1, and c2 > 0 is the time per array element of the div and comb steps...
                figure illustrated one way of figuring out the solution to recurrenec (2.4) 
*/

        //setup a vector for sorting, get the length of it offset for starting at 0, call merge_sort with that vector, the start and end of the vector
        let mut some_vector: Vec<usize> = vec!(3, 4, 2, 1);
        let len = some_vector.len();
        merge_sort(&mut some_vector, 0, len - 1);
        
        fn merge_sort(to_sort: &mut Vec<usize>, start: usize, end: usize) {
                //return out of call if already sorted (partition len of 1)
                if start >= end { return; }
                //get the middle of the partition provided and call this again to keep splitting them up recursively
                let middle = (start + end) / 2;
                merge_sort(to_sort, start, middle);
                merge_sort(to_sort, middle + 1, end);
                //call merges when done splitting since returns will start happening
                merge(to_sort, start, middle, end);
        }
        
        fn merge(to_sort: &mut Vec<usize>, start: usize, middle: usize, end: usize) {
                //the n values for the first and second half of the partition provided for to_sort[start..end], temporary vectors to store values
                let n_left = middle - start + 1;
                let n_right = end - middle;
                let mut left: Vec<usize> = vec!();
                let mut right: Vec<usize> = vec!();
                //storing values into those temporary left and right vectors
                for i in 0..n_left {
                left.push(to_sort[start + i]);
                }
                for i in 0..n_right {
                right.push(to_sort[middle + i + 1]);
                }
                let mut i: usize = 0;       //indexes handled in left vector
                let mut j: usize = 0;       //handled in the right vector
                let mut k: usize = start;   //the position to start placing values in the unsorted vector
                
                //alrighty - if i is left the n for the left vector and j i sstill less than n for the right vector then iterate
                //  ... if the i'th value in left is less than or equal to the j'th value in right, push the i'ith value in left to the vector
                //  ... otherwise do that for the j'th value in right - in both cases increment i or j depending on the branch
                //  ... always increment k by 1 since that is the position in this particular parition of the vector
                while i < n_left && j < n_right {
                if left[i] <= right[j] {
                        to_sort[k] = left[i];
                        i = i + 1;
                } else {
                        to_sort[k] = right[j];
                        j = j + 1;
                }
                k = k + 1;
                }
                // after flip floppin' through any remaining values for n_left go into the array
                while i < n_left {
                to_sort[k] = left[i];
                i = i + 1;
                k = k + 1;
                }
                // and the same for the n_right remaining values
                while j < n_right {
                to_sort[k] = right[j];
                j = j + 1;
                k = k + 1;
                }
                // poof all done
        }
        // adds to vectors filled with u8 integers that are either 0 or 1 without validating
        fn add_binary_integers_little_endian(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
                // set a couple length values since it'll be referred to more than once
                let len_a = a.len();
                let len_b = b.len();
                // fast results
                match (len_a, len_b) {
                        (0, 0) => return vec![],
                        (_, 0) => return a.clone(),
                        (0, _) => return b.clone(),
                        _ => {},
                }
                // figure for iterations by taking the largest length between the two
                let n = len_a.max(len_b);
                // substract the length of each from n to get an offset
                let a_pad = n - len_a;
                let b_pad = n - len_b;
                // set a result to push bits into
                let mut result = Vec::with_capacity(n + 1);
                // set a carry value
                let mut carry = 0;
                //start iterating from the least significant bit
                for i in (0..n).rev() {
                        // compare and offset, or 0, to stay in each vector's index range
                        // 0's fill bits for the the short vector's negative indexes
                        let a_bit = if i >= a_pad { a[i - a_pad] } else { 0 };
                        let b_bit = if i >= b_pad { b[i - b_pad] } else { 0 };
                        // add the bits and the carry value
                        let sum = a_bit + b_bit + carry;
                        carry = sum / 2;
                        result.push(sum % 2);
                }
                if carry > 0 {
                        result.push(1); 
                }
                result.reverse();
                result
        }
        let a: Vec<u8> = vec![1, 0, 0, 0];
        let b: Vec<u8> = vec![1, 0, 0, 0];
        let c = add_binary_integers_little_endian(&a, &b);
        println!("little endian c: {c:?}");


        fn add_binary_integers_big_endian(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
                let len_a = a.len();
                let len_b = b.len();
                let n;
                match (len_a, len_b) {
                        (0, 0) => return vec![],
                        (_, 0) => return a.clone(),
                        (0, _) => return b.clone(),
                        (_, _) => n = len_a.max(len_b),
                }
                let mut result = Vec::with_capacity(n + 1);
                let mut carry = 0;
                for i in 0..n {
                        let a_bit = if len_a > i { a[i] } else { 0 };
                        let b_bit = if len_b > i { b[i] } else { 0 };
                        let sum = a_bit + b_bit + carry;
                        carry = sum / 2;
                        result.push(sum % 2);
                }
                if carry > 0 {
                        result.push(1); 
                }
                result
        }
        let a: Vec<u8> = vec![1, 0, 1, 1, 0, 1];
        let b: Vec<u8> = vec![1, 1, 1, 0, 1, 1];
        let c = add_binary_integers_big_endian(&a, &b);
        println!("big endian c: {c:?}");

        //under flow test
                        println!("test underflow...");
        /* fn linear_search(a: &[usize], n: usize, find: usize) -> bool {
                for val in 0..n {       // init: what is true? the value we are searching for is always itself, and we searching a[0-n]
                        if val == find {       // maint: during the iteration we check if the value at a[val] is equal to the value we are finding, and if it is we terminate
                                return true;    // terminate: when terminating a true or false value should be returned in all cases if the value was or wasn't found
                        }                       //      if iterating and it was found true is returned
                }                               //      if all iters up to n are checked, and no exact match was found then false returns
                false                                   // does explain using loop invariants properly?
        } */

        /* gpt tutor:
                for linear_search loop invariant explanation
                        init: before the loop starts, we know we have not yet found the value in the array yet, and we can then assume that as a true fact
                        maint: during each iteration we check if a[i] is equal to the value we're finding, and if it is we immediately return true
                                these iterations are maintained all the way up to the point of i being n unless they were equal at some point
                        term: the loop terminates when i reaches unless i was found at any index where i < n - if it was found then true would have already been returned
                                at this point we can definitely determine that value we were searching for was not in the array, and false is returned
         */

        /* fn insertion_sort_reverse(a: &mut [usize], n: usize) {
                for i in 1..n + 1 {                     // also add 1 to n here to catch the tail end of the arr
                        let key = a[i];
                        let mut j = i;
                        while j > 0 && a[j - 1] < key {         //just try flipping this greater to a less than first ALMOST correct, off by 1 :(
                                a[j] = a[j - 1];
                                j -= 1;
                        }
                        a[j] = key;
                }
        }

        let mut arr: [usize; 6] = [31, 41, 59, 26, 41, 58];
        println!("before sort: {arr:?}");
        let len = arr.len() - 1;
        insertion_sort_reverse(&mut arr, len);
        println!("afterr sort: {arr:?}"); */

        // fn sum_isize_array(a: &[isize], n: usize) -> isize { //E X E R C I S E - 2.1-2... first loop invariant for i in 0..n+1
        //         let mut sum = 0;                        // invariant for first iter is given and array that starts with 31 would be
        //         for i in 0..n + 1 {                             // i = 31, which would be correct given an array that starts with 31?
        //                 sum += a[i];                                    // in the maintenance portion... we add to the sum at the current ith index of a, but do nothing
        //         }                                                        // the the loop terminates when i reaches n
        //         sum
        // }
        //         /* gpt tutoring:
        //                 what I wrote for sum_isize_array is not right as 31 is just some variant that can be provided
        //                         we need think about what is true for the sum, not about i... so
        //                                 init: before the loop (when i = 0) sum = 0, which is the sum of 0 elements
        //                                 maint: at each step we add a[i] to the sum maintaining that sum is the sum of elements up to i
        //                                 term: when i is eventually n the sum contains the sum of all elements up to n in the array
        //                                         ... these statements hold true throughout the algorithm, and supposes that the algo is correct
        //                                         ... based on these three factors
        //                                                 ... we take the initial state of what's changing
        //                                                 ... we maintain the change that's supposed to happen to that as iterations continue
        //                                                 ... we show that upon termination, the expected changes have occured...
        //         */

        // let arr: [isize; 6] = [31, 41, 59, 26, 41, 58];
        // let len = arr.len() - 1;
        // let result = sum_isize_array(&arr, len);
        // println!("sum: {result:?}; arr summed: {arr:?}");
        
        // so EXERCISE 2.3 expects rewriting insertion sort to sort in descending order, which I can work out, but as a programmer
                // that is absurdly lazy and not trying to optimize this I'd just build an array for each index and send back the reverse order after doing this
                // for optimum speed and to sort into the correct order...
                //      the reverse operations must happpen sooo....
                //              just change the less than to greater than and as it iters shpoop
        /* fn insertion_sort(a: &mut [usize], n: usize) {          // EXERCISE 2.1-1 takes mut array ref, and n iters 
                for i in 1..n {                         //  for i in 1 through n iters
                        let key = a[i];                 // key = the val of array at i
                        let mut j = i;                  // mut j = i as well
                        while j > 0 && a[j - 1] > key {         // if j > 0, and array index [j minus 1] is greater than a[ at index i]
                                a[j] = a[j - 1];                // the j'th index of a = the j'th index - 1
                                j -= 1;                         // j -= 1, subtract 1 from j shifting for the next iter of the while loop
                        }
                        a[j] = key;                             // set the value at a[j] to the k when finishing iter
                }
        }

        let mut arr: [usize; 6] = [31, 41, 59, 26, 41, 58];
        println!("before sort: {arr:?}");
        let len = arr.len() - 1;
        insertion_sort(&mut arr, len);
        println!("afterr sort: {arr:?}"); */

/*        // !!!!!!! LEAVE CODE HERE SO WE CAN FIND PLACE AGAIN THIS IS A LOT OF LINES LOL
        // I would like to do the same maths via code now to get proper answers
        // gawd this implementation...
        use bigdecimal::BigDecimal;
        use bigdecimal::FromPrimitive;
        use bigdecimal::ToPrimitive;
        
        fn lazy_notation(input: BigDecimal) -> String {
                let input_string = input.to_string();
                if input <= BigDecimal::from(1000000) {
                    input_string
                } else {
                    let mut count = input_string.lena() - 1;
                    if let Some(index) = input_string.find('.') {
                        count = index - 1;
                    }
                    let first_digit = input_string[0..1].to_string();
                    let proceeding_three = input_string[1..4].to_string();
                    format!("{first_digit}.{proceeding_three}e+{count}")
                }
            }
            
        let one_second_in_micros = BigDecimal::from_f64(1_000_000.0).unwrap();
        let one_minute_in_micros = &one_second_in_micros * 60;
        let one_hour_in_micros = &one_minute_in_micros * 60;
        let one_day_in_micros = &one_hour_in_micros * 24;
        let one_month_in_days = BigDecimal::from_f64(365.25 / 12.0).unwrap();
        let one_month_in_micros = &one_month_in_days * &one_day_in_micros; //amusingly a whole number
        let one_year_in_micros = &one_month_in_micros * 12;
        let one_century_in_micros = &one_year_in_micros * 100;

        let values_of_n = [
                one_second_in_micros,
                one_minute_in_micros,
                one_hour_in_micros,
                one_day_in_micros,
                one_month_in_micros,
                one_century_in_micros,
        ];
        let in_dur_strs = [
                "in one second",
                "in one minute",
                "in one hour",
                "in one day",
                "in one month",
                "in one year",
        ];

        //log(n)
        println!("----------- runs for log(n)");
        // for (index, n) in values_of_n.iter().enumerate() {
        //         let some_f64_val = n.to_f64();
        //         if let Some(f64_val) = some_f64_val {
        //                 let log = f64_val.log(2.0);
        //                 let notated = lazy_notation(BigDecimal::from_f64(log).unwrap());
        //                 println!("{} {}", notated, in_dur_strs[index]);
        //         }    

// example code goes here */

/* ### COMPLETED SECTIONS GET MOVED DOWN
    ... going over the table of contents
        1 - the role of algos in computing
                algos
                algos as a technology
        2 - getting started
                insertion sort
                analyzing algos
                designing algos
        3 - characterizing run times
                O-notation, Omega-notation, and Theta-notation
                Asymptotic notation: formal definitions
                Standard notations and common functions
        4 - divide-and-conquer
                multiplying square matrices
                strassen's algo for matrix multiplication
                the substitution method for matrix multi
                the recursion-tree method for solving recurrences
                the master method for solving recurrences
                proof of continuous master theorem
                akra-bazzi recurrences
        5 - probabilistic analysis and randomized algorithms
                the hiring problem
                indicator random variables
                randomized algorithms
                probabilistic analysis and further uses of indicator random variables
        6 - heapsort
                heaps
                maintaining the heap property
                building a heap
                the heapsort algo
                priority queues
        7 - quicksort
                description of quicksort
                performance of quicksort
                a randomized version of quicksort
                analysis of quicksort
        8 sorting in linear time
                lower bounds for sorting
                counting sort
                radix sort
                bucket sort
        9 medians and order statistics
                minimum and maximum
                selection in expected linear time
                selection in worst-case linear time
        10 elementary data structures
                simple array-based data structures: arrays, matricies, stacks, queues
                linked lists
                representing rooted trees
        11 hash tables
                direct-address tables
                hash tables
                hash functions
                open addressing
                practical considerations
        12 binary search trees
                what is a binary search tree
                querying a binary search tree
                insertion and deletion
        13 red-black trees
                properties of red-black trees
                rotations
                insertion
                deletion
        14 dynamic programming
                rod cutting
                matrix-chain multiplication
                elements of dynamic programming
                longest common subsequence
                optimal binary search trees
        15 greedy algorithms
                an activity-selection problem
                element of the greedy strategy
                huffman codes
                offline caching
        16 amortized analysis
                aggregate analysis
                the accounting method
                the potential method
                dynamic tables
        17 augmenting data structures
                dynamic order statistics
                how to augment a data structure
                interval trees
        18 b-trees
                definition of b-trees
                basic operations on b-trees
                deleting a key from a b-tree
        19 data structures for disjoint sets
                disjoint-set operations
                linked-list representation of disjoint sets
                disjoint-set forests
                analysis of union by rank with path compression
        20 elementary graph algorithms
                representations of graphs
                breadth-first search
                depth-first search
                topological sort
                strongly connected components
        21 minimum spanning trees
                growing a minimum spanning tree
                the algos of kruskal and prim
        22 single-source shortest paths
                the bellman-ford algorithm
                single-source shortest paths in directed acyclic graphs
                dojkstra's algorithm
                difference constraints and shortest paths
                proofs of shortest-paths properties
        23  all-pairs shortest paths
                shortest paths and matrix multiplication
                the floyd-warshall algorithm
                johnson's algo for sparse graphs
        24 maximum flow
                flow networks
                the ford-fulkerson method
                maximum bipartite matching
        25 matchings in bipartite graphs
                maximum bipartite matching (revisited)
                the stable-marriage problem
                the hungarian algo for the assignment problem
        26 parallel algos
                the basis of fork-join parallelism
                parallel matrix multiplication
                parallel merge sort
        27 online algos
                waiting for an elevator
                maintaining a search list
                online caching
        28 matrix operations
                solving systems of linear equations
                inverting matrices
                symmetric positive-definite matricies and least-squares approximation
        29 linear programming
                linear programming forumations and algorithms
                formulating problems as linear programs
                duality
        30 polynomials and the FFT
                representing polynomials
                the dft and fft
                fft circuits
        31 number-theoretic algos
            elementary number-theoretic notions
            greatest common divisor
            modular arithmetic
            solving modular linear equations
            the chinese remainder theorem
            powers of an element
            the rsa public-key cryptosytem
            primality test
        32 string matching
            the naive string-matching algo
            the rabin-karp algo
            string match with finite automata
            the knuth-morris-pratt algo
            suffix arrays
        33 machine-learning algos
            clustering
            multiplicative-weights algos
            gradient descent
        34 np-completness
            polynomial time
            polynomial time verification
            np completness and reducibility
            np completenes proofs
            np complete problems
        35 approx algos
            the vertex-cover problem
            the traveling salesperson problem
            the set-covering problem
            randomization and linear programming
            the subset-sum problem
        appendix: math background
            A summations
                summation formulas and properties
                bounding summations
            B sets, etc.
                sets
                relations
                functions
                graphs
                trees
            C counting and probability
                counting
                probability
                discrete random variables
                the geometric and binomial distributions
                the tails of binomial distrobution
            D matricies
                matrices and matrix operations  
                basic matrix properties
*/
}

/*
        --- CONQUERED SECTIONS ---
*/