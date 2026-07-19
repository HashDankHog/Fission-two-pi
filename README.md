<h1>Currently, I am in the middle of rewriting my constraint system, so the software is broken</h1>
<h1>The only reason I am pushing this is bc I need the morale boost of seeing the little green checks that pop up showing that I actually have been doing something</h1>
Shi twin ts not complete come back in a couple months

<h1>Dependencies</h1>
<p>make sure you have rust and cargo downloaded.
The front end uses Serde and Tauri, but Cargo should
automatically handle downloading them.</p>

<h1>How To Run</h1>
<h2>Parsing Example</h2>
<p>Open up a terminal and navigate to the cloned repository.
after that run</p>
<code> cd solver
cargo run --example parse_example</code>
<p> The example program supports the following </p>
<ul>
<li> entering expression eg. (2+3/4) will result in
the value of the expression being printed eg. (2.75)
</li>
<li> variables can be declared by typing p<bold>n</bold>
=<bold>expression</bold> where <bold>n</bold> is an integer
</li>
<li>the program currently supports the following operators <ul>
<li>+</li>
<li>-</li>
<li>*</li>
<li>/</li>
<li>^</li>
<li>sin</li>
<li>cos</li>
<li>tan</li>
</ul>
</li>
<li> the screen can be cleared by typing clear and the program can be exited by typing exit </li>
</ul>
<h1>Frontend</h1>
make sure you sure you are in the root directory of the crate, and run
<code> cargo run</code>
<p>this software is in pre pre alpha so when trying to use this just know its going to suck.
the window with a table contains all your parameters that you will use to plot things. p0 is a special variable and its input shouldnt be typed in
click that add segment button and two new parameters will be added, these two new parameters correspond to the x and y values of a paramtric function where
p0 is the input parameter. input the expression you want to plot and then hit the plot button. p0 goes from a range of 0 to 1
you can add other parameters for things like constraints too or you can add other segments. just keep in mind that it will crash if you type something in wrong
and its just a buggy pos rn</p>