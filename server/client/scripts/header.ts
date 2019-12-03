
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

// since for now each page has it's own DOM load event listener
// we need to execute theese when they are loaded
setTimeout(() => {
  initHeaderEventListeners();
})
