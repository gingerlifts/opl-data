#!/usr/bin/env python3

import os
import copy

from datetime import datetime

from oplcsv import CsvReadIter

def opl_path(filename):
    return os.path.join('build', filename)

def map_disamb_lifters(lifters):

    name_i = lifters.index('Name')
    lifter_id_i = lifters.index('LifterID')

    lifter_name_d = {} 
    lifter_id_set = set()

    for lifter_row in lifters:
        lifter_name = lifter_row[name_i]
        lifter_id = lifter_row[lifter_id_i]

        # we only care about disambiguated lifters as they have been
        # manually vetted in

        if lifter_name.find('#') != -1:
            lifter_name_d[lifter_id] = lifter_name
            lifter_id_set.add(lifter_id)

    return (lifter_name_d, lifter_id_set,)

def load_meet_data(meets):

    meet_d = {}

    meet_id_i = meets.index('MeetID')
    date_i = meets.index('Date')
    
    for meet_row in meets:
        meet_id = meet_row[meet_id_i]
        meet_d[meet_id] = datetime.strptime(meet_row[date_i], "%Y-%m-%d")

    return meet_d


def collect_bw_data(entries, meet_d, lifter_id_set):

    bw_delta_list_d = {}
    bw_date_last_entry_d = {}

    meet_id_i = entries.index('MeetID')
    lifter_id_i = entries.index('LifterID')
    bodyweight_i = entries.index('BodyweightKg')

    for entry_row in entries:
        
        lifter_id = entry_row[lifter_id_i] 
        meet_id = entry_row[meet_id_i]
        meet_date = meet_d[meet_id]
        bw_str = entry_row[bodyweight_i]

        # if the entry is for one of our disambiguated lifter, and we have a valid bw, we care
        if lifter_id in lifter_id_set and bw_str != '':

            bw_kg = float(bw_str)

            # if this is the first entry for this lifter,
            # we don't have a delta yet
            last_entry_tup = bw_date_last_entry_d.get(lifter_id)

            if last_entry_tup:
                (last_meet_id, last_meet_date, last_bw_kg,) = last_entry_tup
            
                # if multiple entries for the same meet, ignore
                if last_meet_id != meet_id:

                    if not bw_delta_list_d.get(lifter_id):
                        bw_delta_list_d[lifter_id] = []

                    delta_bw_kg = abs(bw_kg - last_bw_kg)
                    delta_bw_pct = (delta_bw_kg / last_bw_kg) * 100.0

                    meet_date_td = meet_date - last_meet_date
                    delta_days = abs(meet_date_td.days)

                    bw_delta_list_d[lifter_id].append((delta_bw_pct, delta_days,))


            bw_date_last_entry_d[lifter_id] = (meet_id, copy.deepcopy(meet_date), bw_kg,)

    return bw_delta_list_d


def analyse_data(bw_delta_list_d):

    avg_list = []

    # for each lifter, find the average bw pct per day
    for (lifter_id, bw_delta_list,) in bw_delta_list_d.items():
        sum_bw_pct = 0
        sum_days = 0

        for (delta_bw_pct, delta_days,) in bw_delta_list:
            sum_bw_pct += float(delta_bw_pct)
            sum_days += float(delta_days)

        avg_list.append(sum_bw_pct / sum_days)

    return (min(avg_list), max(avg_list),)

if __name__ == '__main__':

    lifters = CsvReadIter(opl_path('lifters.csv'))
    meets = CsvReadIter(opl_path('meets.csv'))
    entries = CsvReadIter(opl_path('entries.csv'))

    (lifter_name_d, lifter_id_set,) = map_disamb_lifters(lifters)

    print("loaded {} disambiguated lifters".format(len(lifter_name_d)))

    meet_d = load_meet_data(meets)

    print("loaded {} meets".format(len(meet_d)))

    bw_delta_list_d = collect_bw_data(entries, meet_d, lifter_id_set)
    (min_avg, max_avg) = analyse_data(bw_delta_list_d)

    print("min average bw%/day: {}".format(min_avg))
    print("max average bw%/day: {}".format(max_avg))

