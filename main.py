#!/usr/bin/env python

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

class Application(Gtk.Window):

    def __init__(self):
        Gtk.Window.__init__(self, title="Application")

        self.set_default_size(200, 200)

win = Application()
win.connect("delete-event", Gtk.main_quit)
win.show_all()
Gtk.main()
