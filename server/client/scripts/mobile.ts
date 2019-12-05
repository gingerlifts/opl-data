function toggleMobileFilters(): void {
    const mobileMenu = document.getElementById("header-mobile-menu") as HTMLDivElement;
    const filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
    // hide mobile menu when user clicks on filters
    if(mobileMenu && mobileMenu.classList) {
      mobileMenu.classList.add("hide");
    }

    if (filtersMobileMenu && filtersMobileMenu.classList) {
      filtersMobileMenu.classList.toggle("hide");
    }
}

function toggleMobileMenu(): void {
    const filtersMobileMenu = document.getElementById("controls-mobile-menu") as HTMLDivElement;
    const mobileMenu = document.getElementById("header-mobile-menu") as HTMLDivElement;
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

  if (desktopControls) {
    desktopControls.innerHTML = '';
  }
}

function initMobileEventListeners(): void {
  const mobileControlsBtn = document.getElementById("controls-mobile-toggle-button") as HTMLButtonElement;
  const mobileMenuToggler = document.getElementById("mobileMenuToggler") as HTMLButtonElement;
  const mobileMenuLinks = document.getElementsByClassName("nav__link_mobile") as HTMLCollection;

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
