let mobileControlsBtn: HTMLButtonElement;
let mobileMenu: HTMLDivElement;
let mobileMenuToggler: HTMLButtonElement;
let mobileMenuLinks: HTMLCollection;
let filtersMobileMenu: HTMLDivElement;

function toggleMobileFilters(): void {
    // hide mobile menu when user clicks on filters
    mobileMenu = document.getElementById("header-mobile-menu") as HTMLDivElement;
    mobileMenu.classList.add("hide");

    mobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
    mobileMenu.classList.toggle("hide");
}

function toggleMobileMenu(): void {
    filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
    mobileMenu = document.getElementById("header-mobile-menu") as HTMLDivElement;
    // hide filters menu when user clicks on main menu
    if (filtersMobileMenu && filtersMobileMenu.classList) {
      filtersMobileMenu.classList.add("hide");
    }
    // toggle mobile menu
    if (mobileMenu && mobileMenu.classList) {
      mobileMenu.classList.toggle("hide");
    }

}

function removeDesktopViews(): void {
  const desktopGlobalMenu = document.getElementById("header") as HTMLButtonElement;
  const desktopControls = document.getElementById("controls") as HTMLButtonElement;
  if (desktopGlobalMenu) {
    desktopGlobalMenu.innerHTML = '';
  }

  if(desktopControls) {
    desktopControls.innerHTML = '';
  }
}

function initMobileEventListeners(): void {
  mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
  mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;
  mobileMenuLinks = document.getElementsByClassName("nav__link_mobile") as HTMLCollection;

  if (mobileControlsBtn) {
    mobileControlsBtn.addEventListener("click", toggleMobileFilters, false);
  }

  if (mobileMenuToggler) {
    mobileMenuToggler.addEventListener("click", toggleMobileMenu, false);
  }

  if (mobileMenuLinks.length > 0) {
    for (let i = 0; i < mobileMenuLinks.length; i++) {
      mobileMenuLinks[i].addEventListener("click", toggleMobileMenu, false);
    }
  }
}

function onMobileLoad(): void {
  removeDesktopViews();
  initMobileEventListeners();
}

export {
  onMobileLoad
}
