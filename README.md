# Rust Particle Simulator

## Description
This is an improvement over the particle simulator I made in C using SDL <br />
There are on screen graphics which show the number of particles and total kinetic energy of the particles <br />
Controls are improved upon

## Controls
C - Toggle collisions (default = <b>true</b>) <br />
G - Toggle gravity (default = <b>false</b>) <br />
Click - Adds particle at mouse position, or applies a velocity towards the mouse depending on mode <br />
Backspace - Remove particle <br />
Shift - Toggle mouse mode

### Modes
Add/Remove mode - Allows the player to add and remove particles <br />
Mouse force mode - Creates a force towards the mouse

## Build
You should be able to build the program using <b>cargo</b> and it will take care of dependencies for you
