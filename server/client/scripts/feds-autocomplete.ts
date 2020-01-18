declare let FEDERATIONS_OPTIONS: Object;
declare let GLOBAL_FEDERATIONS_OPTIONS: Object;

function createOption(text: string, label: string): HTMLElement {
  const option = document.createElement("span");

  option.setAttribute("class", "federation-option");
  option.setAttribute("label", label);
  option.innerHTML = text;

  return option;
}

function createOptionGroup(text: string, label: string): HTMLElement {
  const option = document.createElement("span");

  option.setAttribute("class", "federation-option-group");
  option.setAttribute("label", label);
  option.innerHTML = text;

  return option;
}

function initFedsAutocomplete(this: any, changeSelection): void {
  // get fed select and changeSelection from rankings page
  const callBack = changeSelection;
  const fedsInput = document.getElementById("feds-autocomplete-input") as HTMLInputElement;

  fedsInput.onfocus = showFeds;
  fedsInput.onblur = hideFeds;

  fedsInput.addEventListener('input', filterFeds);
  fedsInput.addEventListener('onkeyup', filterFeds);

  function renderFilteredResults(filter): void {
    const fedsContainer = document.getElementById("feds-autocomplete-results") as HTMLElement;
    const regExp = new RegExp('^' + filter, 'i');
    // clear previous options
    fedsContainer.innerHTML = ""


    for (const globalOption in GLOBAL_FEDERATIONS_OPTIONS) {
      if (filter.length && regExp.test(GLOBAL_FEDERATIONS_OPTIONS[globalOption])) {
        const option = createOption(GLOBAL_FEDERATIONS_OPTIONS[globalOption], globalOption);
        fedsContainer.appendChild(option);
      }
    }

    for (const country in FEDERATIONS_OPTIONS) {
      if (filter.length && regExp.test(country)) {
        const option = createOptionGroup(country, country);
        fedsContainer.appendChild(option);

      }
      for (const fed in FEDERATIONS_OPTIONS[country]) {
        if (filter.length && regExp.test(FEDERATIONS_OPTIONS[country][fed])) {
          const option = createOption(FEDERATIONS_OPTIONS[country][fed], fed);
          fedsContainer.appendChild(option);
        }
      }
    }
  }

  function renderAllResults(): void {
    const fedsContainer = document.getElementById("feds-autocomplete-results") as HTMLElement;
    // clear previous options
    fedsContainer.innerHTML = "";

    for (const globalOption in GLOBAL_FEDERATIONS_OPTIONS) {
      const option = createOption(GLOBAL_FEDERATIONS_OPTIONS[globalOption], globalOption);
      fedsContainer.appendChild(option);
    }

    for (const country in FEDERATIONS_OPTIONS) {
      const option = createOptionGroup(country, country);
      fedsContainer.appendChild(option);

      for (const fed in FEDERATIONS_OPTIONS[country]) {
        const option = createOption(FEDERATIONS_OPTIONS[country][fed], fed);
        fedsContainer.appendChild(option);
      }
    }
  }

  function filterFeds(this: any): void {
    const inputValue = this.value;

    if (inputValue.length > 0) {
      renderFilteredResults(inputValue);
    } else {
      renderAllResults();
    }
    attachClickListeners();
  }

  function attachClickListeners (): void {
    const renderedOptions = document.getElementsByClassName("federation-option");

    for (var i = 0; i < renderedOptions.length; i++) {
        renderedOptions[i].addEventListener('click', (e: any) => {
          const fedsInput = document.getElementById("feds-autocomplete-input") as HTMLInputElement;
          const fedsContainer = document.getElementById("feds-autocomplete-results") as HTMLElement;
          const label = e.target.getAttribute('label');

          // set input value
          fedsInput.value = e.target.innerText;
          // hide options
          fedsContainer.classList.add('hide');

          callBack(label);
        }, false);
    }
  }

  function showFeds(): void {
    const fedsContainer = document.getElementById("feds-autocomplete-results") as HTMLElement;
    const fedsInput = document.getElementById("feds-autocomplete-input") as HTMLInputElement;
    const inputValue = fedsInput.value;

    fedsInput.value = '';
    fedsContainer.classList.remove('hide');

    renderAllResults();
    attachClickListeners();
  }

  function hideFeds(): void {
    const fedsContainer = document.getElementById("feds-autocomplete-results") as HTMLElement;
  }
}

export {
  initFedsAutocomplete
}
