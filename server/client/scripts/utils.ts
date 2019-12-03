const MOBILE_MAX_WIDTH = 767;

function getViewPortSize(): { width: number; height: number; } {
    const doc = document, w = window;
    const docEl = (doc.compatMode && doc.compatMode === 'CSS1Compat')?
            doc.documentElement: doc.body;

    let width = docEl.clientWidth;
    let height = docEl.clientHeight;

    // mobile zoomed in?
    if ( w.innerWidth && width > w.innerWidth ) {
        width = w.innerWidth;
        height = w.innerHeight;
    }

    return { width: width, height: height };
}

function isMobile(): Boolean {
  const screenWidth = getViewPortSize().width as Number;

  return screenWidth <= MOBILE_MAX_WIDTH;
}

export {
  getViewPortSize,
  isMobile,

  MOBILE_MAX_WIDTH
}
