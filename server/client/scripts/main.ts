import { onMobileLoad } from './mobile';
import { loadMeetScripts } from './meet';
import { loadMeetList } from './meetlist';
import { loadRankingsScripts } from './rankings'
import { loadRecordsScripts } from './records';
import { isMobile } from './utils';

function changeLanguage(): void {
   const time = new Date();
   time.setFullYear(time.getFullYear()+3);
   const langselect = document.getElementById("langselect") as HTMLSelectElement;
   const langselectValue = langselect.value || 'en';
   const expireTime = time.toUTCString();

   document.cookie="lang=" + langselectValue + "; expires=" + expireTime + "; path=/; ";
   var h = window.location.href;
   window.location.href = h.substring(0, h.indexOf("?"));
}

function changeUnits(): void {
   const time = new Date();
   time.setFullYear(time.getFullYear()+3);
   const weightunits = document.getElementById("weightunits") as HTMLSelectElement;
   const weightUnitsValue = weightunits.value || 'lbs';
   const expireTime =  time.toUTCString();

   document.cookie="units=" + weightUnitsValue + "; expires=" + expireTime + "; path=/; ";
   window.location.href = window.location.href;
}

function initHeaderEventListeners(): void {
  const weightunits = document.getElementById("weightunits") as HTMLSelectElement;
  const langselect = document.getElementById("langselect") as HTMLSelectElement;

  weightunits.addEventListener("change", changeUnits);
  langselect.addEventListener("change", changeLanguage);
}

export {
  changeLanguage,
  changeUnits
}

document.addEventListener("DOMContentLoaded", () => {
  const thisPage = window.location.pathname;

  // first we check if user is on mobile
  // if yes - remove desktop header and controls
  // since desktop and mobile header and controls have
  // have html elements with same ids - we need first
  // remove desktop elements not ot have duplicated id's
  if(isMobile()) {
      onMobileLoad();
  }

  initHeaderEventListeners();

  if (thisPage.indexOf("/m/") >= 0) {
    loadMeetScripts();
  } else if (thisPage.indexOf("/mlist") >= 0) {
    loadMeetList();
  } else if (thisPage.indexOf("/records") >= 0) {
    loadRecordsScripts();
  } else if(thisPage === '/' || thisPage.indexOf("/rankings") >= 0){
    loadRankingsScripts();
  }
});
