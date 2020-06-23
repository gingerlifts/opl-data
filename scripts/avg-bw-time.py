#!/usr/bin/env python3

import os

from datetime import datetime
from statistics import mean

from oplcsv import CsvReadIter

def opl_path(filename):
    return os.path.join('build', filename)

def map_disamb_lifters(lifters):

    lifter_name_d = {} 
    lifter_id_set = set()

    for lifter_row in lifters:

        # we only care about disambiguated lifters as they have been
        # manually vetted in

        if lifter_row['Name'].find('#') != -1:
            lifter_name_d[lifter_row['LifterID']] = lifter_row['Name']
            lifter_id_set.add(lifter_row['LifterID'])

    return (lifter_name_d, lifter_id_set,)


# augment entries with a datetime object from the date string in meets
# only if it's in the set of lifter IDs we care about
def augment_and_sort_entries(entries, meets, lifter_id_set):

    meet_d = {meet_row['MeetID']: datetime.strptime(meet_row['Date'], "%Y-%m-%d") for meet_row in meets}

    entry_list = []

    for entry_row in entries:
        if entry_row['LifterID'] in lifter_id_set and entry_row['BodyweightKg'] != '':
            entry_row['DateObj'] = meet_d[entry_row['MeetID']]
            entry_list.append(entry_row) 

    sorted_entry_list = sorted(entry_list, key=lambda entry: entry['DateObj'])

    return sorted_entry_list


def collect_bw_data(sorted_entry_list):

    bw_delta_list_d = {}
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
            # 10307,mags/plusa/1982-01-30-D,USPF,1981-11-07,USA,GA,Augusta,Southern Bench Press,
            # 10817,mags/plusa/1981-12-21-A,USPF,1981-11-07,USA,GA,Augusta,Augusta Class II,

            if last_meet_id != meet_id and last_meet_date != meet_date:

                if not bw_delta_list_d.get(lifter_id):
                    bw_delta_list_d[lifter_id] = []

                delta_bw_kg = abs(bw_kg - last_bw_kg)
                delta_bw_pct = (delta_bw_kg / last_bw_kg) * 100.0

                meet_date_td = meet_date - last_meet_date
                delta_days = meet_date_td.days

                # if this is negative then we didn't sort properly
                if delta_days < 0:
                    raise ValueError("delta of days from one meet to next shouldn't be negative if meets were sorted!")

                bw_delta_list_d[lifter_id].append((delta_bw_pct, delta_days,))
                
        # datetime objects are deep copied implicitly
        bw_date_last_entry_d[lifter_id] = (meet_id, meet_date, bw_kg,)


    return bw_delta_list_d


def analyse_data(bw_delta_list_d):

    bw_pct_per_day2_d = {}
    avg_bw_pct_per_day2_d = {}

    # for each lifter, find the bw pct per day and store it against
    # the number of days so we can find the curve of d2(bw_pct) / d(days)
    for (lifter_id, bw_delta_list,) in bw_delta_list_d.items():
        sum_bw_pct = 0
        sum_days = 0

        for (delta_bw_pct, delta_days,) in bw_delta_list:

            if not bw_pct_per_day2_d.get(delta_days):
                bw_pct_per_day2_d[delta_days] = []

            bw_pct_per_day2_d[delta_days].append(delta_bw_pct / float(delta_days))
        
    # now for each day delta find the average bw pct delta
    for (delta_days, bw_pct_per_day_list,) in bw_pct_per_day2_d.items():
        avg_bw_pct_per_day2_d[delta_days] = mean(bw_pct_per_day_list)

    # now sort by delta_days
    sorted_avg_bw_pct_per_day2_items = sorted(avg_bw_pct_per_day2_d.items(), key=lambda item_tup: item_tup[0])

    return sorted_avg_bw_pct_per_day2_items


if __name__ == '__main__':

    lifters = CsvReadIter(opl_path('lifters.csv'), dict_reader=True)
    meets = CsvReadIter(opl_path('meets.csv'), dict_reader=True)
    entries = CsvReadIter(opl_path('entries.csv'), dict_reader=True)

    (lifter_name_d, lifter_id_set,) = map_disamb_lifters(lifters)
    sorted_entry_list = augment_and_sort_entries(entries, meets, lifter_id_set)
    bw_delta_list_d = collect_bw_data(sorted_entry_list)
    sorted_avg_bw_pct_per_day2_items = analyse_data(bw_delta_list_d)

    for (delta_days, avg_bw_pct_per_day,) in sorted_avg_bw_pct_per_day2_items:
        print("Days: {}, Mean bw% difference per day: {}".format(delta_days, avg_bw_pct_per_day))

