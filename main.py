#!/usr/bin/env python

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

class Application(Gtk.Window):

	def __init__(self):
		Gtk.Window.__init__(self, title="Application")

		self.set_default_size(800, 600)

		box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)

		searchentry = Gtk.SearchEntry()
		searchentry.connect("search-changed", self.search)

		searchbar = Gtk.SearchBar()
		searchbar.connect_entry(searchentry)
		searchbar.add(searchentry)

		box.pack_start(searchbar, False, False, 0)

		self.treeview = Gtk.TreeView()

		renderer = Gtk.CellRendererText()
		column = Gtk.TreeViewColumn("Text", renderer, text=0)
		self.treeview.append_column(column)

		scrolled = Gtk.ScrolledWindow()
		scrolled.add(self.treeview)
		box.pack_start(scrolled, True, True, 0)

		self.add(box)

		header = Gtk.HeaderBar()
		header.set_show_close_button(True)
		header.props.title = "Application"
		self.set_titlebar(header)

		searchbutton = Gtk.ToggleButton.new_with_label("Search")
		searchbutton.connect("toggled", lambda button: searchbar.set_search_mode(button.get_active()))
		header.pack_end(searchbutton)

	def search(self, searchentry):
		print searchentry.get_text()

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
