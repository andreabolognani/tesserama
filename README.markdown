Tesserama
=========

Membership cards manager.

It was written with a very specific use case in mind, so it will probably
not be very useful to anyone else.


Requirements
------------

GTK+ 3 is the only runtime requirement. The target distribution is Ubuntu
16.04 LTS; anything newer than that should work just fine.


Compilation
-----------

Tesserama can be built using `cargo`, but it's expected that compilation
will happen through Flatpak. The provided `Makefile` can be used to both
build (`make`) and test (`make run`) the software in a convenient way.


Installation
------------

The recommended installation method is through Flatpak.

    $ flatpak remote-add kiyuko.org --from https://kiyuko.org/flatpak/repo
    $ flatpak install kiyuko.org org.kiyuko.Tesserama

The above assumes both Flatpak and Flathub (where the GNOME runtime, used
by Tesserama, is hosted) have been configured properly. For information on
how to do that, see the respective websites.


Limitations
-----------

Although fully working, this is basically a prototype: more specifically,
error checking is not performed nearly as extensively as it should, so for
example trying to load an invalid file will trash the application.

Some features are not discoverable, only partially by design.


Future work
-----------

Address the limitations listed above.


Resources
---------

The canonical Git repository can be found at

  https://git.kiyuko.org/tesserama


License
-------

This program is free software; you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation; either version 2 of the License, or (at your option) any later
version.
