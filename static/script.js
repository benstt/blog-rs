const SMALL_WINDOW_SIZE = 575;

const setSmallViewportLinks = (lis, anchors) => {
    // Change every HTML of the item to be the text with the link
    for (let i = 0; i < lis.length; i++) {
        const text = lis[i].getElementsByTagName("strong")[0].innerText;
        const anchorLink = anchors[i].href;

        lis[i].innerHTML = `<a href="${anchorLink}">${text}</a>`;
    }
}

const setBigViewportLinks = (lis, anchors) => {
    // Revert every item to its original form
    for (let i = 0; i < lis.length; i++) {
        const text = lis[i].getElementsByTagName("a")[0].innerText;
        const anchorLink = anchors[i].href;

        lis[i].innerHTML = `<strong>${text}</strong>: <a href="${anchorLink}">${anchorLink}</a>`;
    }
}

const lis = document.getElementById("links").getElementsByTagName("li");
const anchors = document.getElementById("links").getElementsByTagName("a");

let alreadySetToSmall = false;
if (screen.width < SMALL_WINDOW_SIZE || window.innerWidth < SMALL_WINDOW_SIZE) {
    setSmallViewportLinks(lis, anchors);
    alreadySetToSmall = true;
}

window.addEventListener('resize', () => {
    if (screen.width < SMALL_WINDOW_SIZE || window.innerWidth < SMALL_WINDOW_SIZE) {
        if (!alreadySetToSmall) {
            alreadySetToSmall = true;
            setSmallViewportLinks(lis, anchors);
        }
    } else {
        if (alreadySetToSmall) {
            alreadySetToSmall = false;
            setBigViewportLinks(lis, anchors);
        }
    }
})