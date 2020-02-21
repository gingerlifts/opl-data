// vim: set ts=4 sts=4 sw=4 et:
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

// Implementation of main logic for the Meet page.

'use strict';

// These are generated inline via templates/meet.html.tera.
declare const path_if_by_ah: string;
declare const path_if_by_division: string;
declare const path_if_by_dots: string;
declare const path_if_by_glossbrenner: string;
declare const path_if_by_ipfpoints: string;
declare const path_if_by_mcculloch: string;
declare const path_if_by_nasa: string;
declare const path_if_by_reshel: string;
declare const path_if_by_schwartzmalone: string;
declare const path_if_by_total: string;
declare const path_if_by_wilks: string;

let selSort: HTMLSelectElement;

// When selectors are changed, the URL in the address bar should
// change to match.
function redirect() {
    switch (selSort.value) {
        case "by-ah":
            window.location.href = path_if_by_ah;
            break;
        case "by-division":
            window.location.href = path_if_by_division;
            break;
        case "by-dots":
            window.location.href = path_if_by_dots;
            break;
        case "by-glossbrenner":
            window.location.href = path_if_by_glossbrenner;
            break;
        case "by-ipf-points":
            window.location.href = path_if_by_ipfpoints;
            break;
        case "by-mcculloch":
            window.location.href = path_if_by_mcculloch;
            break;
        case "by-nasa":
            window.location.href = path_if_by_nasa;
            break;
        case "by-reshel":
            window.location.href = path_if_by_reshel;
            break;
        case "by-schwartz-malone":
            window.location.href = path_if_by_schwartzmalone;
            break;
        case "by-total":
            window.location.href = path_if_by_total;
            break;
        case "by-wilks":
            window.location.href = path_if_by_wilks;
            break;
    }
}

function toggleDetails(this: any): void {
  const hiddenRows = document.getElementsByClassName('results-lifter-hidden-row') as HTMLCollection;
  const arrows = document.getElementsByClassName('results-lifter-more-arrow') as HTMLCollection;

  if (hiddenRows.length && arrows.length) {
    if (this.checked) {
        for (let row of hiddenRows) {
          row.classList.remove("hide");
        }

        for (let arrow of arrows) {
          arrow.classList.add("rotate");
        }

    } else {
      for (let row of hiddenRows) {
        row.classList.add("hide");
      }
      for (let arrow of arrows) {
        arrow.classList.remove("rotate");
      }
    }
  }
}

function toggleDivisions(this: any): void {
  if (this.checked) {
      selSort.value = "by-division";
      redirect();
  } else {
      selSort.value = "by-wilks";
      redirect();
  }
}

function initTogglers(): void {
  const divisionToggler = document.getElementById("byDivisionToggler") as HTMLInputElement;
  const showDetailsToggler = document.getElementById('showDetailsToggler') as HTMLInputElement;

  if (showDetailsToggler) {
    showDetailsToggler.addEventListener('change', toggleDetails);
  }
  if (divisionToggler) {
    divisionToggler.addEventListener('change',toggleDivisions);
  }
}

function initCategoriesTogglers(): void {
  const categories = document.getElementsByClassName('category') as HTMLCollection;

  if (categories.length) {
    for (let category of categories) {
      const categoryHeader = category.querySelector('.divheader') as HTMLElement;
      const hiddenData = category.querySelector('.results-group-row') as HTMLElement;
      const arrow = category.querySelector('img') as HTMLElement;

      categoryHeader.addEventListener('click', function() {
        hiddenData.classList.toggle("hide");
        arrow.classList.toggle("rotate");
      });
    }
  }
}

function initLifterDetailsTogglers(): void {
  const lifterRows = document.getElementsByClassName('results-lifter-row-container') as HTMLCollection;

  if (lifterRows.length) {
    for (let row of lifterRows) {
      const hiddenData = row.querySelector('.results-lifter-hidden-row') as HTMLElement;
      const arrow = row.querySelector('img') as HTMLElement;

      row.addEventListener('click', function() {
        hiddenData.classList.toggle("hide");
        arrow.classList.toggle("rotate");
      });
    }
  }
}

function initMeet(): void {
    initTogglers();
    initCategoriesTogglers();
    initLifterDetailsTogglers();

    selSort = document.getElementById("sortselect") as HTMLSelectElement;
    selSort.addEventListener("change", redirect);
}

export {
    initMeet
}
