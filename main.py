#!/usr/bin/env python

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

class Application(Gtk.Window):

	def __init__(self):
		Gtk.Window.__init__(self, title="Application")

		self.set_default_size(800, 600)

		self.treeview = Gtk.TreeView()

		renderer = Gtk.CellRendererText()
		column = Gtk.TreeViewColumn("Text", renderer, text=0)
		self.treeview.append_column(column)

		scrolled = Gtk.ScrolledWindow()
		scrolled.add(self.treeview)

		self.add(scrolled)

	def load_data(self):

		self.data = Gtk.ListStore(str)

		with open("COPYING", "rb") as f:
			for line in f:
				self.data.append([line.strip()])

		self.treeview.set_model(self.data)

win = Application()
win.load_data()

win.connect("delete-event", Gtk.main_quit)
win.show_all()
Gtk.main()
