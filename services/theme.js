// Match the CV's stored theme and its light default. Content and navigation
// remain available when JavaScript or local storage is disabled.
(function () {
    try {
        const theme = window.localStorage.getItem("cv-theme");
        if (theme === "dark" || theme === "light") {
            document.documentElement.setAttribute("data-theme", theme);
        }
    } catch (_) {}
})();
