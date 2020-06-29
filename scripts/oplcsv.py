#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# A better CSV manipulation library for the OpenPowerlifting format.
#

import codecs


class Csv:

    def __init__(self, filename=None):
        if filename:
            with open(filename, 'r', encoding='utf-8') as fd:
                self.fieldnames = fd.readline().rstrip().split(',')
                self.rows = [x.rstrip("\r\n").split(',')
                             for x in fd.readlines()]
        else:
            self.fieldnames = []
            self.rows = []

    def __len__(self):
        return len(self.rows)

    def index(self, name):
        return self.fieldnames.index(name)

    def append_column(self, name):
        self.fieldnames.append(name)
        for row in self.rows:
            row.append('')

    def append_columns(self, namelist):
        self.fieldnames += namelist
        addend = ['' for x in namelist]
        for row in self.rows:
            row += addend

    def insert_column(self, index, name):
        self.fieldnames.insert(index, name)
        for row in self.rows:
            row.insert(index, '')

    def remove_column_by_index(self, idx):
        del self.fieldnames[idx]
        for row in self.rows:
            del row[idx]

    def remove_column_by_name(self, name):
        for i, header in enumerate(self.fieldnames):
            if header == name:
                self.remove_column_by_index(i)
                return

    def remove_empty_columns(self):
        for i in range(len(self.fieldnames)):
            empty = True
            for row in self.rows:
                if row[i]:
                    empty = False
                    break
            if empty:
                self.remove_column_by_index(i)
                self.remove_empty_columns()
                return

    # Integrate another Csv object into the current one.
    def cat(self, other):
        for header in other.fieldnames:
            if header not in self.fieldnames:
                self.append_column(header)

        # An array mapping index in other.fieldnames to index in
        # self.fieldnames.
        mapping = [self.index(header) for header in other.fieldnames]

        for row in other.rows:
            build = ['' for x in range(0, len(self.fieldnames))]

            for i, cell in enumerate(row):
                build[mapping[i]] = cell

            self.rows.append(build)

    def write(self, fd):
        fd.write(','.join(self.fieldnames) + "\n")
        for row in self.rows:
            fd.write(','.join(row) + "\n")

    def write_filename(self, filename):
        ''' Specifies UTF-8 codec. Necessary for Debian Python3. '''
        with codecs.open(filename, 'w', encoding="utf-8") as fd:
            self.write(fd)

class CsvReadIter:

    def __init__(self, filename=None, fd=None, dict_reader=False):
        if filename and not fd:
            self.fd = open(filename, 'r', encoding='utf-8')
        elif fd:
            self.fd = fd
        else:
            raise ValueError("Need at least filename or fd")

        self.dict_reader = dict_reader

        # call next() rather than .readline() so it works with any iterator that
        # iterates over lines
        self.fieldnames = next(self.fd).rstrip().split(',')

    def __iter__(self):
        return self

    def __next__(self):
        next_line = next(self.fd)

        if next_line == '':
            self.fd.close()
            raise StopIteration

        next_row = next_line.rstrip("\r\n").split(',')
        
        if self.dict_reader:
            return {self.fieldnames[i]: v for (i, v,) in enumerate(next_row)}

        return next_row

    def next(self):
        return self.__next__()

    def index(self, name):
        return self.fieldnames.index(name)

