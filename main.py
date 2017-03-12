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
		searchentry.connect("stop-search", lambda entry: self.set_search_mode(False))

		self.searchbar = Gtk.SearchBar()
		self.searchbar.connect_entry(searchentry)
		self.searchbar.add(searchentry)

		box.pack_start(self.searchbar, False, False, 0)

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

		self.searchbutton = Gtk.ToggleButton.new_with_label("Search")
		self.searchbutton.connect("toggled", lambda button: self.set_search_mode(button.get_active()))
		header.pack_end(self.searchbutton)

	def set_search_mode(self, mode):
		self.searchbutton.set_active(mode)
		self.searchbar.set_search_mode(mode)

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
