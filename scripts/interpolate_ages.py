#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Uses existing age to data to estimate ages for all
# lifter meets. Only fills in data if it is consistent.

import sys


def to_string(f):
    try:
        return "{:.2f}".format(f)
    except ValueError:
        print("Field not a float: %f" % f, file=sys.stderr)
        sys.exit(1)


def is_int(s):
    try:
        int(s)
        return True
    except ValueError:
        return False


# Check that a lifter has a consistent birthyear
def is_by_consistent(lifter_data):
    if lifter_data[0][0] != '':
        # Want the lower age if age is derived from birthyear
        age = float(lifter_data[0][0]) - (float(lifter_data[0][0]) % 1)
    else:
        age = -1

    minage = lifter_data[0][1]
    maxage = lifter_data[0][2]
    agedate = lifter_data[0][3]
    mindate = lifter_data[0][3]
    maxdate = lifter_data[0][3]

    if len(lifter_data) > 1:
        for age_data in lifter_data[1:]:
            newagedate = age_data[3]
            new_year = int(age_data[3][:4])

            new_mindate = age_data[3]
            new_minage = age_data[1]
            new_maxdate = age_data[3]
            new_maxage = age_data[2]
            newage = -1
            if age_data[0] != '':
                # Want the lower age if age is derived from birthyear
                newage = float(age_data[0]) - (float(age_data[0]) % 1)
            else:
                newage = -1

            ageyeardiff = new_year - int(agedate[:4])
            if newagedate[4:] < agedate[4:]:
                ageyeardiff -= 1
            minyeardiff = new_year - int(mindate[:4])
            if new_mindate[4:] < mindate[4:]:
                minyeardiff -= 1
            maxyeardiff = new_year - int(maxdate[:4])
            if new_maxdate[4:] < maxdate[4:]:
                maxyeardiff -= 1

            # Check that the age is consistent
            if newage != -1:
                if age != -1 and newage < age + ageyeardiff:
                    return False
                elif newage < minage + minyeardiff:
                    return False
                elif newage > maxage + maxyeardiff + 1:
                    return False

            # Check that the minage is consistent
            if new_minage != 0 and new_minage > maxage + maxyeardiff+1:
                return False

            # Check that the maxage is consistent
            if new_maxage != 999 and new_maxage < minage + minyeardiff:
                return False

            if newage != -1:
                age = newage
                agedate = newagedate
            if new_minage != 0:
                minage = new_minage
                mindate = new_mindate
            if new_maxage != 999:
                maxage = new_maxage
                maxdate = new_maxdate

    return True


def mnth_day_cmp(age_data):
    return age_data[1][4:]


# Checks whether a lifter has a consistent birthday
def is_bd_consistent(lifter_data):
    for age_data in lifter_data:
        age = age_data[0]
        date = age_data[3]

        bd_data = []

        if age != '':
            # this is an exact age
            if float(age) % 1 == 0:
                bd_data.append([age, date])

    if len(bd_data) > 1:
        # Sort the age data by day and month
        bd_data.sort(mnth_day_cmp)

        init_year = bd_data[0][1][:4]

        # Offset the age data so it is all from one year
        for age_date in bd_data[1:]:
            curr_year = age_date[1][:4]
            age_date[0] += init_year - curr_year

        # Check that the age data is still sorted by age,
        # if not the birthdate isn't consistent
        for ii in range(1, len(bd_data)):
            if bd_data[ii-1][0] > bd_data[ii][0]:
                return False

    return True


# Gives the range that a lifters birthday lies in
def estimate_birthdate(lifter_data):
    min_date = ''
    max_date = ''
    bd_data = []
    for age_data in lifter_data:
        age = age_data[0]
        date = age_data[3]

        # this is an exact age
        if age != '' and float(age) % 1 == 0:
            bd_data.append([int(age), date])

    if len(bd_data) > 1:
        # Sort the age data by day and month
        bd_data.sort(key=mnth_day_cmp)

        init_year = int(bd_data[0][1][:4])

        # Offset the age data so it is all from one year
        for age_date in bd_data[1:]:
            curr_year = int(age_date[1][:4])
            age_date[0] += init_year - curr_year

        min_date = bd_data[0][1][5:]
        max_date = bd_data[0][1][5:]
        has_had_bd = False
        lower_age = bd_data[0][0]

        for age_date in bd_data[1:]:
            if age_date[0] == lower_age:
                min_date = age_date[1][5:]
            elif age_date[0] == lower_age + 1:
                max_date = age_date[1][5:]
                has_had_bd = True

                break

        # We can't estimate a birthdate
        if not has_had_bd:
            return []

        else:  # We've managed to bound the birthdate
            by = init_year - (lower_age + 1)
            min_date = str(by)+'-'+min_date
            max_date = str(by)+'-'+max_date
            return [min_date, max_date]
    else:  # No recorded ages, can't estimate a birthdate
        return []


# Gets the range where we know that the lifter hasn't had a birthday
# this function assumes that there are no years where we see the lifter at different ages
def get_known_range(lifter_data):
    min_date = ''
    max_date = ''
    bd_data = []
    for age_data in lifter_data:
        age = age_data[0]
        date = age_data[3]

        # this is an exact age
        if age != '' and float(age) % 1 == 0:
            bd_data.append([int(age), date])

    if len(bd_data) > 1:
        # Sort the age data by day and month
        bd_data.sort(key=mnth_day_cmp)

        init_year = int(bd_data[0][1][:4])

        # Offset the age data so it is all from one year
        for age_date in bd_data[1:]:
            curr_year = int(age_date[1][:4])
            age_date[0] += init_year - curr_year

        min_date = bd_data[0][1][5:]
        max_date = bd_data[0][1][5:]

        for age_date in bd_data[1:]:
            if age_date[1][5:] < min_date:
                min_date = age_date[1][5:]
            elif age_date[1][5:] > max_date:
                max_date = age_date[1][5:]

        return [max_date, min_date]
    elif len(bd_data) == 1:
        return [bd_data[0][1][5:], bd_data[0][1][5:]]
    else:
        return []


def interpolate_lifter(lifter_data):

    if len(lifter_data) > 1:
        bd_range = estimate_birthdate(lifter_data)

        if bd_range != []:  # Then we have a birthday range and can be semi-accurate
            for age_data in lifter_data:
                if age_data[0] == '':
                    mnth_day = age_data[3][4:]
                    # Then they haven't had their birthday yet
                    if mnth_day < bd_range[0][4:]:
                        age_data[0] = int(age_data[3][:4]) - \
                            int(bd_range[0][:4])-1
                        age_data[1] = age_data[0]
                        age_data[2] = age_data[0]
                    # Then they've had their birthday
                    elif mnth_day > bd_range[1][4:]:
                        age_data[0] = int(age_data[3][:4])-int(bd_range[0][:4])
                        age_data[1] = age_data[0]
                        age_data[2] = age_data[0]
                    else:  # We're not sure if they've had their birthday
                        age_data[0] = int(age_data[3][:4]) - \
                            int(bd_range[0][:4])-0.5
                        age_data[1] = int(age_data[0]-0.5)
                        age_data[2] = int(age_data[0]+0.5)
        else:  # We have only birthyears, a single age or only divisions

            by = 0
            approx_by = 0
            min_by = 0
            max_by = 9999
            known_range = []
            # Extract all the birthyear information possible

            for age_data in lifter_data:
                # Then we have an age derived from birthyear
                if age_data[0] != '':
                    if float(age_data[0]) % 1 == 0.5:
                        by = int(age_data[3][:4]) - \
                            int((float(age_data[0])+0.5))
                    else:
                        approx_by = int(age_data[3][:4])-int(age_data[0])

                # Find the tighest bounds given by divisions
                if max_by != 9999 and int(age_data[3][:4]) - age_data[1] < max_by:
                    max_by = int(age_data[3][:4]) - int(age_data[1])

                if min_by != 0 and int(age_data[3][:4]) - age_data[2] > min_by:
                    min_by = int(age_data[3][:4]) - int(age_data[2])

            # Then the division information let's us have an exact birthyear
            if min_by > approx_by:
                by = min_by
            elif max_by < approx_by:
                by = max_by

            # If we have at least one exact age,
            # then we have a range in which we know they don't have a birthday
            if approx_by != 0:
                known_range = get_known_range(lifter_data)

            # First deal with the case when we have a birthyear
            if by != 0:
                for age_data in lifter_data:
                    if age_data[0] == '' or float(age_data[0]) % 1 == 0.5:
                        if known_range == []:
                            age_data[0] = int(age_data[3][:4])-by-0.5
                            age_data[1] = int(age_data[0]-0.5)
                            age_data[2] = int(age_data[0]+0.5)
                        else:
                            # Check whether known_range is an upper
                            # or lower bound on the birthday
                            lower_bound = False
                            if approx_by < by:
                                lower_bound = True

                            # Then the lifter hasn't had their birthday yet
                            if lower_bound and age_data[3][5:] <= known_range[1]:
                                age_data[0] = int(age_data[3][:4])-by - 1
                                age_data[1] = age_data[0]
                                age_data[2] = age_data[0]
                            # Then we're not sure if they've had their birthday
                            elif lower_bound and age_data[3][5:] > known_range[1]:
                                age_data[0] = int(age_data[3][:4])-by-0.5
                                age_data[1] = int(age_data[0]-0.5)
                                age_data[2] = int(age_data[0]+0.5)
                            # Then the lifter has had their birthday
                            elif age_data[3][5:] >= known_range[0]:
                                age_data[0] = int(age_data[3][:4])-by
                                age_data[1] = age_data[0]
                                age_data[2] = age_data[0]
                            # Then we're not sure if they've had their birthday
                            else:
                                age_data[0] = int(age_data[3][:4])-by-0.5
                                age_data[1] = int(age_data[0]-0.5)
                                age_data[2] = int(age_data[0]+0.5)

            # Then deal with the case where we have an age
            # and the division information doesn't give the birthyear
            elif approx_by != 0:

                # Assign upper and lower age bounds based on approximate birthyear
                for age_data in lifter_data:
                    year = int(age_data[3][:4])
                    if age_data[0] == '':
                        if age_data[3][5:] < known_range[0]:
                            age_data[0] = year - approx_by - 0.5
                            age_data[1] = year - approx_by - 1
                            age_data[2] = year - approx_by
                        elif age_data[3][5:] > known_range[1]:
                            age_data[0] = year - approx_by + 0.5
                            age_data[1] = year - approx_by
                            age_data[2] = year - approx_by + 1
                        # We know an exact age for this date
                        elif (age_data[3][5:] >= known_range[0] and
                              age_data[3][5:] <= known_range[1]):
                            age_data[0] = year - approx_by
                            age_data[1] = year - approx_by
                            age_data[2] = year - approx_by

            # Finally deal with the only division case
            else:
                # Set age bounds based on divisions
                for age_data in lifter_data:
                    year = int(age_data[3][:4])
                    if min_by != 0:
                        age_data[1] = year - min_by - 1
                    if max_by != 9999:
                        age_data[2] = year - max_by

    return lifter_data


def interpolate_ages(LifterAgeHash, MeetDateHash):

    for lifter in LifterAgeHash:
        # Create an array of age data sorted by date
        lifter_data = []
        for age_data in LifterAgeHash[lifter]:
            lifter_data.append(
                age_data[:3]+[MeetDateHash[age_data[3]]]+[age_data[3]])

        lifter_data.sort(key=lambda x: x[3])

        if is_by_consistent(lifter_data) and is_bd_consistent(lifter_data):
            lifter_data = interpolate_lifter(lifter_data)

        lifter_data.sort(key=lambda x: x[4])

        # Put this data back into the hashmap
        for ii in range(len(LifterAgeHash[lifter])):
            LifterAgeHash[lifter][ii][0] = lifter_data[ii][0]
            LifterAgeHash[lifter][ii][1] = lifter_data[ii][1]
            LifterAgeHash[lifter][ii][2] = lifter_data[ii][2]

    return LifterAgeHash


def generate_hashmap(entriescsv, meetcsv):
    # Hashtable for lifter age-data lookup
    # int -> [(str, int, int, int),....] LifterID -> Array of (Age,MinAge,MaxAge,MeetID).
    LifterAgeHash = {}
    # Hashtable for looking up meet-dates from IDs, int -> str
    MeetDateHash = {}

    lifterIDidx = entriescsv.index('LifterID')
    ageidx = entriescsv.index('Age')
    minageidx = entriescsv.index('MinAge')
    maxageidx = entriescsv.index('MaxAge')
    meetIDidx = entriescsv.index('MeetID')

    for row in entriescsv.rows:
        lifterID = int(row[lifterIDidx])
        age = row[ageidx]
        minage = int(row[minageidx])
        maxage = int(row[maxageidx])
        meetID = int(row[meetIDidx])

        if lifterID not in LifterAgeHash:
            LifterAgeHash[lifterID] = [[age, minage, maxage, meetID]]
        else:
            LifterAgeHash[lifterID].append([age, minage, maxage, meetID])

    meetIDidx = meetcsv.index('MeetID')
    dateidx = meetcsv.index('Date')

    for row in meetcsv.rows:
        date = row[dateidx]
        meetID = int(row[meetIDidx])

        MeetDateHash[meetID] = date

    return [LifterAgeHash, MeetDateHash]


def get_ageclass(maxage, minage):

    if maxage <= 18:
        return '0-18'
    elif maxage <= 23:
        return '19-23'
    elif minage >= 39 and maxage <= 44:
        return '39-44'
    elif minage > 44 and maxage <= 49:
        return '45-49'
    elif minage > 49 and maxage <= 54:
        return '50-54'
    elif minage > 54 and maxage <= 59:
        return '55-59'
    elif minage > 59 and maxage <= 64:
        return '60-64'
    elif minage > 64 and maxage <= 69:
        return '65-69'
    elif minage > 69 and maxage <= 74:
        return '70-74'
    elif minage > 74 and maxage <= 79:
        return '75-79'
    elif minage > 79:
        return '80-999'
    else:
        return ''

# Adds the interpolated ages back to the csv file, removes MinAge & MaxAge


def update_csv(entriescsv, LifterAgeHash):

    lifterIDidx = entriescsv.index('LifterID')
    ageidx = entriescsv.index('Age')
    meetIDidx = entriescsv.index('MeetID')

    if 'AgeClass' not in entriescsv.fieldnames:
        entriescsv.append_column('AgeClass')

    ageclassidx = entriescsv.index('AgeClass')

    for row in entriescsv.rows:
        lifterID = row[lifterIDidx]
        meetID = row[meetIDidx]

        for age_data in LifterAgeHash[int(lifterID)]:
            if age_data[3] == int(meetID):

                row[ageidx] = str(age_data[0])
                row[ageclassidx] = str(get_ageclass(age_data[1], age_data[2]))
                break

    entriescsv.remove_column_by_name("MinAge")
    entriescsv.remove_column_by_name("MaxAge")

    return entriescsv


def interpolate(entriescsv, meetcsv):
    [LifterAgeHash, MeetDateHash] = generate_hashmap(entriescsv, meetcsv)
    LifterAgeHash = interpolate_ages(LifterAgeHash, MeetDateHash)

    return update_csv(entriescsv, LifterAgeHash)
