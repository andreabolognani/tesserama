#!/usr/bin/env python

import os.path

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

class Application(Gtk.Window):

	def __init__(self):
		Gtk.Window.__init__(self)

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

		self.headerbar = Gtk.HeaderBar()
		self.headerbar.set_show_close_button(True)
		self.set_titlebar(self.headerbar)

		self.searchbutton = Gtk.ToggleButton()
		self.searchbutton.set_image(Gtk.Image.new_from_icon_name("edit-find-symbolic", Gtk.IconSize.BUTTON))
		self.searchbutton.set_tooltip_text("Search")
		self.searchbutton.connect("toggled", lambda button: self.set_search_mode(button.get_active()))
		self.headerbar.pack_end(self.searchbutton)

		self.openbutton = Gtk.Button.new_from_icon_name("document-open-symbolic", Gtk.IconSize.BUTTON)
		self.openbutton.set_tooltip_text("Open")
		self.openbutton.connect("clicked", lambda _: self.open_button_clicked())
		self.headerbar.pack_start(self.openbutton)

	def set_search_mode(self, mode):
		self.searchbutton.set_active(mode)
		self.searchbar.set_search_mode(mode)

	def search(self, searchentry):

		filtered = Gtk.ListStore(str)
		needle = searchentry.get_text().lower()

		for item in self.data:
			if needle in item[0].lower():
				filtered.append([item[0]])

		self.treeview.set_model(filtered)

	def open_button_clicked(self):

		dialog = Gtk.FileChooserDialog("Choose a file", self, Gtk.FileChooserAction.OPEN,
		                               (Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
		                                Gtk.STOCK_OPEN, Gtk.ResponseType.OK))

		if dialog.run() == Gtk.ResponseType.OK:
			self.set_filename(dialog.get_filename())
			self.load_data()

		dialog.destroy()

	def set_filename(self, filename):
		self.filename = os.path.abspath(os.path.realpath(filename))

	def load_data(self):

		self.data = Gtk.ListStore(str)

		with open(self.filename, "rb") as f:
			for line in f:
				self.data.append([line.strip()])

		self.treeview.set_model(self.data)

		self.headerbar.props.title = os.path.basename(self.filename)
		self.headerbar.props.subtitle = os.path.dirname(self.filename)


win = Application()

win.connect("delete-event", Gtk.main_quit)
win.show_all()
Gtk.main()
