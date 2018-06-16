#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# USAPL has unfortunately stopped posting individual meet result spreadsheets,
# and now uploads everything to this usapl.liftingdatabase.com service.
# This script taken a liftingdatabase.com URL and converts the results to
# the OpenPowerlifting internal format. It also creates the directory.
#


from bs4 import BeautifulSoup
import errno

import os
import sys
import urllib.request
import re

try:
    from oplcsv import Csv
    import usernames
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv
    import usernames

namehash = {}



def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read()


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)



def makeentriescsv(soup):
    # The page contains everything we'd need to know, except for Sex. Sigh.
    # That always has to be done manually, since men single-event can come
    # after women, although men usually are placed higher.
    csv = Csv()
    csv.append_column('Place')
    csv.append_column('Name')
    csv.append_column('Sex')
    csv.append_column('Event')
    csv.append_column('Division')
    csv.append_column('WeightClassKg')
    csv.append_column('Equipment')
    csv.append_column('BirthYear')
    csv.append_column('Team')
    csv.append_column('State')
    csv.append_column('BodyweightKg')
    csv.append_column('Squat1Kg')
    csv.append_column('Squat2Kg')
    csv.append_column('Squat3Kg')
    csv.append_column('Bench1Kg')
    csv.append_column('Bench2Kg')
    csv.append_column('Bench3Kg')
    csv.append_column('Deadlift1Kg')
    csv.append_column('Deadlift2Kg')
    csv.append_column('Deadlift3Kg')
    csv.append_column('TotalKg')

    table = soup.find("table", {"id": "competition_view_results"})
    table = table.find("tbody")

    state_sex = ''
    state_event = 'SBD'
    state_division = None
    state_equipment = None

    for row in table.find_all('tr'):
        k = len(row.find_all('td'))
        if k == 0:
            state_sex = ''
            # This is a control row, changing some state.
            s = row.find('th').text.strip()
            if s == 'Powerlifting':
                state_event = 'SBD'
            elif s == 'Squat':
                state_event = 'S'
            elif s == 'Bench press':
                state_event = 'B'
            elif s == 'Deadlift':
                state_event = 'D'
            elif s == 'Push Pull':
                state_event = 'BD'
            elif s == 'DD':
                state_event = 'DRUGTEST'

            else:
                (state_division, state_equipment) = ('','')
                if 'Female' in s:
                    state_sex = 'F'
                elif 'Male' in s:
                    state_sex = 'M'

        elif k == 19:
            # This is a results row.
            assert state_event is not None
            assert state_division is not None
            assert state_equipment is not None

            cells = row.find_all('td')
            weightclasskg = cells[0].text.replace('-', '').strip()
            place = cells[1].text.replace('.', '').strip()
            name = cells[2].text.replace('Jr.', 'Jr').replace(
                'Sr.', 'Sr').replace('  ', ' ').strip()
            birthyear = cells[3].text.strip()
            team = cells[4].text.strip()
            state = cells[5].text.strip()
            bodyweightkg = cells[6].text.strip()

            squat1kg = cells[7].text.strip()
            squat2kg = cells[8].text.strip()
            squat3kg = cells[9].text.strip()
            bench1kg = cells[10].text.strip()
            bench2kg = cells[11].text.strip()
            bench3kg = cells[12].text.strip()
            deadlift1kg = cells[13].text.strip()
            deadlift2kg = cells[14].text.strip()
            deadlift3kg = cells[15].text.strip()
            totalkg = cells[16].text.strip()
            # wilks = cells[17].text.strip() # Not used. Always recalculated.
            # drugtested = cells[18].text.strip() # Not used.

            row = ['' for x in csv.fieldnames]
            row[csv.index('WeightClassKg')] = weightclasskg
            row[csv.index('Place')] = place
            row[csv.index('Name')] = name
            row[csv.index('Sex')] = state_sex
            row[csv.index('BirthYear')] = birthyear
            row[csv.index('Team')] = team
            row[csv.index('State')] = state
            row[csv.index('BodyweightKg')] = bodyweightkg
            row[csv.index('Squat1Kg')] = squat1kg
            row[csv.index('Squat2Kg')] = squat2kg
            row[csv.index('Squat3Kg')] = squat3kg
            row[csv.index('Bench1Kg')] = bench1kg
            row[csv.index('Bench2Kg')] = bench2kg
            row[csv.index('Bench3Kg')] = bench3kg
            row[csv.index('Deadlift1Kg')] = deadlift1kg
            row[csv.index('Deadlift2Kg')] = deadlift2kg
            row[csv.index('Deadlift3Kg')] = deadlift3kg
            row[csv.index('TotalKg')] = totalkg
            row[csv.index('Division')] = state_division
            row[csv.index('Equipment')] = state_equipment
            row[csv.index('Event')] = state_event

            for i, c in enumerate(row):
                row[i] = c.replace(',', ' ')
            csv.rows.append(row)

        else:
            cells = row.find_all('td')
            for i, cell in enumerate(cells):
                print("%d: %s" % (i, cell))
            error("Unexpected row length: %s, debug information above" % str(k))

    return csv


# Creates a hashmap keyed on misspellings.
def load_names(datafilepath):
    h = {}
    with open(datafilepath, 'r') as fd:
        for line in fd.readlines():
            names = line.split(',')
            assert len(names) >= 2

            correct_name = names[0].strip()
            assert correct_name

            for incorrect_name in names[1:]:
                incorrect_name = incorrect_name.strip()
                assert incorrect_name
                assert incorrect_name not in h
                h[incorrect_name] = correct_name

    return h


def correct_names(csv):
    global namehash
    assert 'Name' in csv.fieldnames

    nameidx = csv.index('Name')

    for row in csv.rows:
        if row[nameidx] in namehash:
            row[nameidx] = namehash[row[nameidx]]

    return csv






def add_by(oldcsv,sourcecsv):
    old_name_idx = oldcsv.index('Name')
    source_name_idx = sourcecsv.index('Name')
    source_by_idx = sourcecsv.index('BirthYear')

    if 'BirthYear' not in oldcsv.fieldnames:
        oldcsv.append_column('BirthYear')

    old_by_idx = oldcsv.index('BirthYear')

    for row in oldcsv.rows:
        name = usernames.get_username(row[old_name_idx])

        old_by = row[old_by_idx]
        if old_by == '':
            success = False
            for sourcerow in sourcecsv.rows:
                source_name = sourcerow[source_name_idx].lower()
                source_name = re.sub('".*"','',source_name)
                source_name = re.sub('\(.*\)','',source_name)
                if source_name[:3] == 'jr ':
                    source_name = source_name[3:]+" jr"
                source_name = usernames.get_username(source_name.strip())
                source_by = sourcerow[source_by_idx]
                if source_name == name:
                    row[old_by_idx]=source_by
                    success = True
                    break
            if not success:
                print("Couldn't match name for lifter: %s" % name)
    return oldcsv



def add_birthyears(entriespath,url):
    try:
        html = gethtml(url)
        if 'usapl.liftingdatabase.com' in url:
            oldcsv = Csv(entriespath)

            soup = BeautifulSoup(html, 'html.parser')

            sourcecsv = makeentriescsv(soup)
            sourcecsv = correct_names(sourcecsv)
            print('Adding BirthYear to %s' %entriespath)
            oldcsv = add_by(oldcsv,sourcecsv)
            with open(dirname + os.sep + 'entries.csv', 'w') as fd:
                oldcsv.write(fd)
    except:
        print("Couldn't open url for %s" % entriespath)
        pass



if __name__ == '__main__':
    namehash = load_names('../../lifter-data/name-corrections.dat')

    for dirname, subdirs, files in os.walk(os.getcwd()):

        if 'entries.csv' in files:
            entriespath = dirname + os.sep + 'entries.csv'
            urlpath = dirname +os.sep + 'URL'
            if os.path.isfile(urlpath): 
                url_file = open(urlpath,'r')

                add_birthyears(entriespath,url_file.readline())
                url_file.close()