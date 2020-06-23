#!/usr/bin/env python3

import os

from datetime import datetime

from oplcsv import Csv

def opl_path(filename):
    return os.path.join('build', filename)

def map_disamb_lifters(lifters):

    name_i = lifters.index('Name')
    lifter_id_i = lifters.index('LifterID')

    lifter_name_d = {} 

    for lifter_row in lifters.rows:
        cur_name = lifter_row[name_i]
        cur_id = lifter_row[lifter_id_i]

        # we only care about disambiguated lifters as they have been
        # manually vetted in

        if cur_name.find('#') != -1:
            lifter_name_d[cur_id] = cur_name

    return lifter_name_d

def load_meet_data(meets):

    meet_d = {}

    meet_id_i = meets.index('MeetID')
    date_i = meets.index('Date')
    
    for meet_row in meets.rows:
        meet_id = meet_row[meet_id_i]
        meet_d[meet_id] = datetime.strptime(meet_row[date_i], "%Y-%m-%d")

    return meet_d


def collect_bw_data(entries, meet_d):

    bw_delta_list_d = {}
    bw_date_last_entry_d = {}

    meet_id_i = entries.index('MeetID')
    lifter_id_i = entries.index('LifterID')
    bodyweight_i = entries.index('BodyweightKg')

    for entry_row in entries.rows:
        
        lifter_id = entry_row[lifter_id_i] 
        meet_date = meet_d[lifter_id]
        bw_kg = entry_row[bodyweight_i]

        # if this is the first entry for this lifter,
        # save bw and date for comparison to the next one
        if not bw_date_last_entry_d.get(lifter_id):
            bw_date_last_entry_d[lifter_id] = (meet_date, bw_kg,)

        else:
            if not bw_delta_list_d.get(lifter_id):
                bw_delta_list_d[lifter_id] = []

            #MARK 



if __name__ == '__main__':

    lifters = Csv(opl_path('lifters.csv'))
    meets = Csv(opl_path('meets.csv'))
    entries = Csv(opl_path('entries.csv'))

    lifter_name_d = map_disamb_lifters(lifters)
    meet_d = load_meet_data(meets)


