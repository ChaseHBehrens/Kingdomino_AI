# Kingdomino AI
This is an AI that plays the board game kingdomino. 
## How it works
It uses a Greedy algorithm to select the best scoring domino on a given turn. It considers all possible placements of all available domino choices and calculates the score of each corresponding board state. 
## Performance
The AI came third in the both four player matches against humans I tested it with. This was not surprising based on the relative simplicity of the algorithm. 
## Improvements 
This was my first rust program and thus there are several improvements to the structure that would increase readability and maintainability. I would like to implement a better UI and standard engine protocol system so that multiple AI agents could be run against each other. Additionally, some probabilistic calculation for future turns would improve the AI's performance. I plan on implementing a version 2 based on what I've learned from this version.
<br><br>
![image](https://github.com/user-attachments/assets/17ee2982-ed03-483a-b83a-817b639c5966)
## Note 
Unfortunately colors do not seem to work for the windows exe. I originally developed this one in a Linux debian environment.
