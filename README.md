Rusty-door [![Build Status](https://travis-ci.org/rusty-door/rusty-door.svg?branch=master)](https://travis-ci.org/rusty-door/rusty-door)
==========

![concept](/misc/concept.jpg?raw=true "Concept art")

This is the course work in Computer Graphics. Its goal is a 3D dungeon crawler
game.

Naturally, the game must be completed in one semester. Thus, its development
must be performed incrementally so that even if (to be more precise, *when*) we
bust the deadlines, there is still a game we can show.

We need then to define multiple milestones which, when reached, denote a
complete game which can still be very far from what we wish to see but is
playable. Each of the reached milestones shall be assigned a version number.

Milestones
----------

### ASCII labyrinth

A game with the interface provided by [ncurses library][ncurses]. The screen
contains a timer which counts the number of seconds since the start of the
current session, and a field.

The field has two types of tiles, wall tiles and floor tiles, representing a
labyrinth. It is procedurally-generated and as such created anew each time a
new game is started. In the left bottom corner— the «start»— of the labyrinth
is a symbol which can be controlled by the player— the «PC». The PC can move
up, down, left, or right. The moves are instantaneous. The goal is to put the
PC in the right top corner— the «exit». Each labyrinth is guaranteed to only
have a single path from start to the exit which doesn't involve backtracking.

The floor tiles which have been stepped upon an even number of times and those
that have been walked on an odd number of times are displayed differently.
Thus, by the end of the game, according to the uniqueness of the correct path,
it's highlighted.

The game allows pausing it, saving and loading its state with at most a single
save file per user, and displaying the GNU General Public License.

The game also allows to keep the high scores, the sorting criteria being the
number of steps which have been reverted in the course of labyrinth traversal
and the time spent to find the exit.

The program only supports terminal with box-drawing characters.

### Frame buffer labyrinth

This is an extension of the previous game. The logic is exactly the same, the
differences only concern the display facilities.

The interface is a window from the [GLUT
toolkit](GLUT). The only part of
the rendering in which OpenGL participates, however, is flushing the frame
buffer onto the screen (see
[glDrawPixels](https://www.opengl.org/sdk/docs/man2/xhtml/glDrawPixels.xml),
[glClear](https://www.opengl.org/sdk/docs/man2/xhtml/glClear.xml), and
[glSwapBuffers](https://www.opengl.org/sdk/docs/man2/xhtml/glXSwapBuffers.xml)).
The buffers themselves are prepared completely on the client side.

The wall and floor tiles aren't solid colors but have static sprites. The PC,
too, has sprites.

When the PC walks, the sprites change in cycle of five possible states. The PC
sprite has a side which is aligned with the direction of walking— the «face» of
the sprite. When the PC stops, it assumes the idle state. Obviously, the moves
aren't instantaneous but have a certain speed. If the player attempts to move
the PC in the direction not aligned with the face of the PC, then the PC starts
to move without delay, rotating to resume the correct position in the process.

Every ten cells the walls have torches. They are static sources of light. The
PC and the walls have a shadow determined by a simple ray-casting algorithm for
every torch in sight. The wall and floor tiles have differing brightness
depending on the nearby light sources. The brightness isn't determined for a
whole tile but the change is gradually distributed across it so that the border
between tiles isn't obvious.

A thread is drawn from the start to the current position of the PC by
connecting the floor tiles that have been stepped upon an odd number of times.

When the PC reaches the exit, the thread brightens up, while the labyrinth
fades.

### Top-down hack & slash

This one extends the preceding game as well, but this time not only the display
but the logic, too, is different.

First and foremost, there are now a number of monsters present. Their
appearance, behaviour, and difficulty varies. The proposed monster variations
are:

  * large, fat and slow, with heavy attacks and much health,
  * skinny and tall, attacking from the distance, with little health,
  * bulky and short, with medium attack strength and health,
  * tiny flying orbs with as little health as possible, non-aggressive, but
    healing other enemies from time to time.

All of them are 3D models with the following animations:

  * _Idle_: the monster stands, performing some idle motions;
  * _Idle, walking_: the monster walks lo and fro;
  * _Aggressive_: the monster has detected the PC and changes its position;
  * _Charge_: the monster performs the primary battle motion;
  * _Restore_: the monster suffers from the PC's attack.

The PC is a 3D model as well, with animations:

  * _Idle_: the PC stands;
  * _Run_: the PC runs;
  * _Attack_: the PC attacks (probably with three distinct animation
    sequences);
  * _Restore_: the PC was attacked by a monster.

Each monster has a field of sight, and if the PC is in it, the monster is put
into the aggressive mode and never returns to the idle mode.

When murdered, monsters drop golden coins and healing potions. The PC must step
on them in order to collect.

The high score system is updated with the gold counter.

A unit of space is defined to be the PC's height.

The labyrinth is also 3D. Walls have torches attached to them every three
units. The labyrinth is slightly more complex: in addition to corridors, now
one unit wide, rooms are present, varying from 3 to 4 units, to 5 to 8 units.
A room can have multiple exits. The requirement of the existence of the unique
path from start to exit is weakened to the overall requirement of existence.

The rooms are more likely to contain monsters than corridors.

If, in order to display some segment of a floor tile, the camera must see
through a wall, then the wall is displayed as a glowing outline of its borders.

The rendering is performed via calls to [OpenGL][OpenGL]. The screen displays
the field, health bar, gold counter and the timer described for the preceding
milestones.

[ncurses]: https://github.com/jeaye/ncurses-rs
[GLUT]:    https://www.opengl.org/resources/libraries/glut/
[OpenGL]:  https://www.opengl.org/ 

