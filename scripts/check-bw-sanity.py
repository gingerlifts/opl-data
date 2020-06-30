#!/usr/bin/env python3

import os
import re

from datetime import datetime

from oplcsv import CsvReadIter


def bw_pct_delta_sanity(delta_days):

    # from scipy.optimize.curve_fit() in bw-analysis.py
    # y = (30.78803553780311/(2.557537199034035 * x)) + 0.15000000000000002

    # a = 30.78803553780311
    # b = 2.557537199034035
    # c = 0.15000000000000002

    # these were decided on by looking at the scatterplot in calc,
    # adding some critical points around where y flattens to
    # a table in desmos, and then fiddling a, b, and c until I got
    # a curve that just skimmed the top of the data

    a = 25
    b = 1
    c = 0.15

    limit_bw_pct_per_day = (a / (b * delta_days)) + c

    return limit_bw_pct_per_day


def opl_path(filename):
    return os.path.join('build', filename)


def map_lifters(lifters):

    lifter_name_d = {}
    lifter_id_set = set()

    for lifter_row in lifters:

        # we don't care about initialed lifters or single name lifters
        if not (
                re.match(r'^[A-Z]\.\s[A-Z][a-z]+.*$', lifter_row['Name']) and
                lifter_row['Name'].find(' ') != -1
                ):
            lifter_name_d[lifter_row['LifterID']] = lifter_row['Name']
            lifter_id_set.add(lifter_row['LifterID'])

    return (lifter_id_set, lifter_name_d,)


# augment entries with a datetime object from the date string in meets
# only if it's in the set of lifter IDs we care about
def augment_and_sort_entries(entries, meets, lifter_id_set):

    print("constructing date objects for meets")

    meet_d = {
            meet_row['MeetID']: datetime.strptime(meet_row['Date'], "%Y-%m-%d")
            for meet_row in meets
            }

    entry_list = []

    print("augmenting entries with date objects")

    for entry_row in entries:
        if entry_row['LifterID'] in lifter_id_set and entry_row['BodyweightKg'] != '':

            # try only storing what we need from the entry

            entry_list.append({
                'MeetID': entry_row['MeetID'],
                'LifterID': entry_row['LifterID'],
                'BodyweightKg': entry_row['BodyweightKg'],
                'DateObj': meet_d[entry_row['MeetID']],
            })

    print("sorting entries by date objects")

    sorted_entry_list = sorted(entry_list, key=lambda entry: entry['DateObj'])

    return sorted_entry_list


def check_bw_data(sorted_entry_list):

    warning_d = {}
    bw_date_last_entry_d = {}

    for entry_row in sorted_entry_list:

        lifter_id = entry_row['LifterID']
        meet_id = entry_row['MeetID']
        meet_date = entry_row['DateObj']
        bw_str = entry_row['BodyweightKg']

        bw_kg = float(bw_str)

        # if this is the first entry for this lifter,
        # we don't have a delta yet
        last_entry_tup = bw_date_last_entry_d.get(lifter_id)

        if last_entry_tup:
            (last_meet_id, last_meet_date, last_bw_kg,) = last_entry_tup

            # if multiple entries for the same meet, ignore
            # sometimes we also get the same lifter in different meets on the same day
            # eg
            # 10307,mags/plusa/1982-01-30-D,USPF,1981-11-07,USA,GA,Augusta,Southern Bench
            # 10817,mags/plusa/1981-12-21-A,USPF,1981-11-07,USA,GA,Augusta,Augusta Class

            if last_meet_id != meet_id and last_meet_date != meet_date:

                delta_bw_kg = abs(bw_kg - last_bw_kg)
                delta_bw_pct = (delta_bw_kg / last_bw_kg) * 100.0

                meet_date_td = meet_date - last_meet_date
                delta_days = meet_date_td.days

                # if this is negative then we didn't sort properly
                if delta_days < 0:
                    raise ValueError(
                            "delta of days shouldn't be negative if meets were sorted!"
                            )

                # check against what the sane limit of bw% / days is for this
                # delta_days
                if (delta_bw_pct / float(delta_days)) > bw_pct_delta_sanity(delta_days):

                    if not warning_d.get(lifter_id):
                        warning_d[lifter_id] = []

                    warning_d[lifter_id].append((bw_kg, last_bw_kg, delta_days,))

        # datetime objects are deep copied implicitly
        bw_date_last_entry_d[lifter_id] = (meet_id, meet_date, bw_kg,)

    return warning_d


if __name__ == '__main__':

    lifters = CsvReadIter(opl_path('lifters.csv'), dict_reader=True)
    meets = CsvReadIter(opl_path('meets.csv'), dict_reader=True)
    entries = CsvReadIter(opl_path('entries.csv'), dict_reader=True)

    print("map lifters")
    (lifter_id_set, lifter_name_d,) = map_lifters(lifters)

    print("augment and sort entries")
    sorted_entry_list = augment_and_sort_entries(entries, meets, lifter_id_set)

    print("check bodyweight data")
    warning_d = check_bw_data(sorted_entry_list)

    for (lifter_id, warning_tup_list,) in warning_d.items():
        print("{}:".format(lifter_name_d[lifter_id]))

        for (bw_kg, prev_bw_kg, delta_days,) in warning_tup_list:
            print("from {} to {} in {} days".format(prev_bw_kg, bw_kg, delta_days))

    print('------------------------------------------------------------')
    print('Lifters flagged: {}'.format(len(warning_d.keys())))
