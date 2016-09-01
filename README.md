Rusty-door
==========

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
toolkit](https://www.opengl.org/resources/libraries/glut/). The only part of
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

[ncurses]: https://github.com/jeaye/ncurses-rs

