# 3d_fps

A Rust project to create a 3D DOOM style engine.

![Screenshot 2](screenshots/s2.png)

![Screenshot 1](screenshots/s1.png)

![Screenshot 3](screenshots/s3.png)

## Features

3d_fps boasts:

 - Perspective-correct texture-mapped walls, ceilings, and floors
 - A Duke-Nukem style 'portal' based renderer
 - Arbitrarily high-resolution texture support
 - Recreated DOOM 'drifter' movement
 - The lowest FPS you've ever seen

## About

The engine is based on the ideas put forward in
[Bisqwit's video](https://youtu.be/HQYsFshbkYw)
(but isn't a direct port of his code).

This was a great learning project for me, not
least because DOOM 2 is one of my favourite
games of all time.

I plan to eventually add online deathmatch.

However, there is a problem. It is incredibly
slow. On my Macbook, it can manage around 4 FPS
tops, 16 on some other machines I've tried. By
the time I'd got the engine to this state over
a week or so, I didn't feel like debugging this,
but I'll come back to it.
