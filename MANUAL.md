# User Manual
## Getting Started
To start writing Sigma16 code, click on the "New" button at the top left of the window.
This will open a new code editor where you can enter Sigma16 code.
It also opens the code runner for the code.

### Assembling and Running
To assemble a Sigma16 program, click the "reset" button on the code runner.
Given the code is formatted correctly, it will assemble the Sigma16 code into byte code and initialise the interpreter to the "Step" state.
At this point you can click "Step"" to step through the assembled program line-by-line.
Once stepped through, you can click "Step Back" to step backwards through the program.

To run the program to completion, click the drop-down state menu on the code runner and select "Run".
This will put the interpreter into the "Running" state.
You can then press run to execute the program to completion.

### Viewing Interpreter State
On the code runner, click the toggle box on the top right side of the runner window.
This will open the data flow interface for the interpreter.
This windows contains the contents of the interpreter.
As you step through the program, register contents will be highlighted green to indicate they have been modified and red to indicate they have been accessed.
Memory contents will highlight green when then have been modified.

### Editing Multiple Programs
It is possible to edit and run multiple Sigma16 programs simultaneously.
Simply repeat the steps in Getting Started.

## Exercises
There are some pre-made programs included under the "Load Exercises" drop-down menu.
These can be used for testing the functionality of the program.

## Saving Progress
All elements of the UI are saved automatically every 5 to 10 seconds.

## Closing and Deleting Programs
When you click the "Close button" on the code editor the program is not deleted, it can be retrieved from the "Load" drop-down menu.

To delete a program, click the "Delete" button on the code editor, you prompted to confirm you want to delete the program.
