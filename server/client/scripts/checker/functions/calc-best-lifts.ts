// vim: set ts=2 sts=2 sw=2 et:
//
// This file is part of OpenPowerlifting, an open archive of powerlifting data.
// Copyright (C) 2019 The OpenPowerlifting Project.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Automatically calculates the Best3SquatKg, Best3BenchKg, and Best3DeadliftKg columns

"use strict";

import { Csv, csvString } from "../csv";

const float2 = (s: string): number => {
  if (s == "") return 0.0;
  s = s.replace(/ /g, "");
  return Number(s);
};

const addBestColumn = (csv: Csv, lift: string): string => {
  const columnName = `Best3${lift}Kg`;
  if (csv.index(columnName) < 0) csv.appendColumn(columnName);

  const idx = csv.index(columnName);
  const idx1 = csv.index(`${lift}1Kg`);
  if (idx1 < 0) return `Missing column '${lift}1Kg.'\n`;
  const idx2 = csv.index(`${lift}2Kg`);
  if (idx2 < 0) return `Missing column '${lift}2Kg.'\n`;
  const idx3 = csv.index(`${lift}3Kg`);
  if (idx3 < 0) return `Missing column '${lift}3Kg.'\n`;

  const eventIndex = csv.index("Event");

  for (let i = 0; i < csv.rows.length; ++i) {
    const event = csv.rows[i][eventIndex];
    // Check if Event contains the first letter of the lift
    if (event.includes(lift.charAt(0))) {
      let best = "";
      for (const id of [idx1, idx2, idx3]) {
        if (float2(csv.rows[i][id]) > float2(best)) best = csv.rows[i][id];
      }
      if (float2(best) > 0) csv.rows[i][idx] = csvString(best);
    } else {
      csv.rows[i][idx] = "";
    }
  }
  return "";
};

// Creates a new Csv file with the Best3{Lift}Kg recalculated.
//
// On success, returns the new Csv.
// On failure, returns a string describing the error.
export const csvCalcBestLifts = (source: Csv): Csv | string => {
  const csv = source.shallowClone();

  if (csv.index("Event") < 0) return "Missing column 'Event'";

  let s: string = "";

  if (csv.index("Squat1Kg") >= 0) s += addBestColumn(csv, "Squat");
  if (csv.index("Bench1Kg") >= 0) s += addBestColumn(csv, "Bench");
  if (csv.index("Deadlift1Kg") >= 0) s += addBestColumn(csv, "Deadlift");
  if (s) return s;
  return csv;
};
