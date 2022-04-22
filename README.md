Pairing interview tasks

# Tic Tac Toe game

Before your interview, write a program that lets two humans play a game of Tic Tac Toe in a terminal. The program should let the players take turns to input their moves. The program should report the outcome of the game.

During your interview, you will pair on adding support for a computer player to your game. You can start with random moves and make the AI smarter if you have time.

# Lisp parser

Write code that takes some Lisp code and returns an abstract syntax tree. The AST should represent the structure of the code and the meaning of each token. For example, if your code is given "(first (list 1 (+ 2 3) 9))", it could return a nested array like ["first", ["list", 1, ["+", 2, 3], 9]].

During your interview, you will pair on writing an interpreter to run the AST. You can start by implementing a single built-in function (for example, +) and add more if you have time.

# Space Invaders

Write a game of Space Invaders that has computer-controller enemies that move left and right automatically and a human-controlled player that you can move left and right with the arrow keys.

During your interview, you can add the ability to shoot bullets at the enemies and track your score.

# Database server

Before your interview, write a program that runs a server that is accessible on http://localhost:4000/. When your server receives a request on http://localhost:4000/set?somekey=somevalue it should store the passed key and value in memory. When it receives a request on http://localhost:4000/get?key=somekey it should return the value stored at somekey.

During your interview, you will pair on saving the data to a file. You can start with simply appending each write to the file, and work on making it more efficient if you have time.

# Symbolic differentiator

Write a symbolic differentiator for polynomial expressions of one variable.

During your interview, you can add pretty printing to display the results in a more human-friendly format or extend the code to handle more complex expressions.

# Your own project

If you have an existing project that you are particularly excited about working on for your pairing interview, you can submit that. Ideally, the project should be something you've written from scratch (i.e., not using a framework).

If you submit an existing project, please say what you would like to pair on during your interview. You can choose a bug to fix, a feature to add, or a refactor to do. The interview is only thirty minutes long, so pick something that could be reasonably accomplished in twenty minutes. Please also try to choose a project that won't require your interviewer to have a lot of context or domain-specific knowledge, since you don't want to spend your entire interview just trying to explain what your code does.

Lastly, if you do choose your own task, please don't pick something too simple. Our goals for this interview are to understand where you are as a programmer, how you write code, and how you work through problems. If you choose a task that only requires writing one or two lines of code or which you can complete in just a couple of minutes, we likely won't learn what we need to learn from the interview to admit you.
