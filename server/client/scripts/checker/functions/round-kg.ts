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

// Rounds lifting Kg columns to the nearest 0.5kg.

'use strict';

import { Csv, csvString } from "../csv";


// Creates a new Csv file with the attempts rounded.
// On success, returns the new Csv.
// On failure, returns a string describing the error.
export const csvRound = (source: Csv): Csv | string => {
  const csv = source.shallowClone();

  const Squat1Index = csv.index("Squat1Kg");
  const Squat2Index = csv.index("Squat2Kg");
  const Squat3Index = csv.index("Squat3Kg");
  const Squat4Index = csv.index("Squat4Kg");
  const Best3SquatIndex = csv.index("Best3SquatKg");
  const Bench1Index = csv.index("Bench1Kg");
  const Bench2Index = csv.index("Bench2Kg");
  const Bench3Index = csv.index("Bench3Kg");
  const Bench4Index = csv.index("Bench4Kg");
  const Best3BenchIndex = csv.index("Best3BenchKg");
  const Deadlift1Index = csv.index("Deadlift1Kg");
  const Deadlift2Index = csv.index("Deadlift2Kg");
  const Deadlift3Index = csv.index("Deadlift3Kg");
  const Deadlift4Index = csv.index("Deadlift4Kg");
  const Best3DeadliftIndex = csv.index("Best3Deadlift4Kg");
  const TotalIndex = csv.index("TotalKg");


  for (let ii = 0; ii < csv.rows.length; ++ii) {

    if (Squat1Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Squat1Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Squat1Kg' row ${ii+1}: '${csv.rows[ii][Squat1Index]}' not a number`;
      }
      csv.rows[ii][Squat1Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Squat2Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Squat2Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Squat2kg' row ${ii+1}: '${csv.rows[ii][Squat2Index]}' not a number`;
      }
      csv.rows[ii][Squat2Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Squat3Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Squat3Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Squat3Kg' row ${ii+1}: '${csv.rows[ii][Squat3Index]}' not a number`;
      }
      csv.rows[ii][Squat3Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Squat4Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Squat4Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Squat4Kg' row ${ii+1}: '${csv.rows[ii][Squat4Index]}' not a number`;
      }
      csv.rows[ii][Squat4Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Best3SquatIndex > 0) {
      let LiftAsNum = Number(csv.rows[ii][Best3SquatIndex]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Best3SquatKg' row ${ii+1}: '${csv.rows[ii][Best3SquatIndex]}' not a number`;
      }
      csv.rows[ii][Best3SquatIndex] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Bench1Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Bench1Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Bench1Kg' row ${ii+1}: '${csv.rows[ii][Bench1Index]}' not a number`;
      }
      csv.rows[ii][Bench1Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Bench2Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Bench2Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Bench2kg' row ${ii+1}: '${csv.rows[ii][Bench2Index]}' not a number`;
      }
      csv.rows[ii][Bench2Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Bench3Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Bench3Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Bench3Kg' row ${ii+1}: '${csv.rows[ii][Bench3Index]}' not a number`;
      }
      csv.rows[ii][Bench3Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Bench4Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Bench4Index]) 
      if (isNaN(LiftAsNum)) {
      return `Error in 'Bench4Kg' row ${ii+1}: '${csv.rows[ii][Bench4Index]}' not a number`;
      }
      csv.rows[ii][Bench4Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Best3BenchIndex > 0) {
      let LiftAsNum = Number(csv.rows[ii][Best3BenchIndex]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Best3BenchKg' row ${ii+1}: '${csv.rows[ii][Best3BenchIndex]}' not a number`;
      }
      csv.rows[ii][Best3BenchIndex] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Deadlift1Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Deadlift1Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Deadlift1Kg' row ${ii+1}: '${csv.rows[ii][Deadlift1Index]}' not a number`;
      }
      csv.rows[ii][Deadlift1Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Deadlift2Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Deadlift2Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Deadlift2kg' row ${ii+1}: '${csv.rows[ii][Deadlift2Index]}' not a number`;
      }
      csv.rows[ii][Deadlift2Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Deadlift3Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Deadlift3Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Deadlift3Kg' row ${ii+1}: '${csv.rows[ii][Deadlift3Index]}' not a number`;
      }
      csv.rows[ii][Deadlift3Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Deadlift4Index > 0) {
      let LiftAsNum = Number(csv.rows[ii][Deadlift4Index]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Deadlift4Kg' row ${ii+1}: '${csv.rows[ii][Deadlift4Index]}' not a number`;
      }
      csv.rows[ii][Deadlift4Index] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (Best3DeadliftIndex > 0) {
      let LiftAsNum = Number(csv.rows[ii][Best3DeadliftIndex]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'Best3DeadliftKg' row ${ii+1}: '${csv.rows[ii][Best3DeadliftIndex]}' not a number`;
      }
      csv.rows[ii][Best3DeadliftIndex] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
    if (TotalIndex > 0) {
      let LiftAsNum = Number(csv.rows[ii][TotalIndex]) 
      if (isNaN(LiftAsNum)) {
        return `Error in 'TotalKg' row ${ii+1}: '${csv.rows[ii][TotalIndex]}' not a number`;
      }
      csv.rows[ii][TotalIndex] = (0.5*Math.round(2*LiftAsNum)).toString()
    }
  }


  return csv;
};
