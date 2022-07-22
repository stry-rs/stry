// https://github.com/selfawarestudio/marth
var utils;
(function (utils) {
    var call = function (xs, fn) { return [].concat(xs).map(fn); };
    var classes = function (action) { return function (xs, cns) { return call(xs, function (x) {
        var _a;
        return (_a = x.classList)[action].apply(_a, cns.split(' '));
    }); }; };
    utils.id = function (selector) { return document.getElementById(selector); };
    utils.add = function (xs, cns) {
        classes('add')(xs, cns);
    };
    utils.remove = function (xs, cns) {
        classes('remove')(xs, cns);
    };
    utils.toggle = function (xs, cn, force) {
        call(xs, function (x) { return x.classList.toggle(cn, force); });
    };
    var events = function (action) { return function (xs, t, fn, opts) { return call(xs, function (x) { return x["".concat(action, "EventListener")](t, fn, opts); }); }; };
    utils.on = function (xs, t, fn, opts) {
        events('add')(xs, t, fn, opts);
        return function () { return events('remove')(xs, t, fn, opts); };
    };
})(utils || (utils = {}));
window.__STATE__ = {
    navOpen: false
};
(function () {
    var navbarButton = utils.id("navbar-button");
    var navbarButtonIconOpen = utils.id("navbar-button__icon--open");
    var navbarButtonIconClose = utils.id("navbar-button__icon--close");
    if (navbarButton != null && navbarButtonIconOpen != null && navbarButtonIconClose != null) {
        utils.on(navbarButton, "click", function () {
            window.__STATE__.navOpen = !window.__STATE__.navOpen;
        });
    }
    else {
        console.error("required navbar elements are missing");
    }
})();
