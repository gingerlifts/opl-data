import { onMobileLoad } from './mobile';
import { loadMeetScripts } from './meet';
import { loadMeetList } from './meetlist';
import { loadRankingsScripts } from './rankings'
import { loadRecordsScripts } from './records';
import { isMobile } from './utils';
import { initHeaderEventListeners } from './global';

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

  if (thisPage.indexOf("/openipf") >= 0) {
    if (thisPage.indexOf("openipf/m/") >= 0) {
      loadMeetScripts();
    } else if (thisPage.indexOf("openipf/mlist") >= 0) {
      loadMeetList();
    } else if (thisPage.indexOf("openipf/records") >= 0) {
      loadRecordsScripts();
    } else if(thisPage === '/dist/openipf' ||
              thisPage === '/dist/openipf/' ||
              thisPage.indexOf("openipf/rankings") >= 0){
      loadRankingsScripts();
    }
    return;
  }

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
