const COORDS_LS = 'coords'

function saveCoords(positionObject) {
    localStorage.setItem(COORDS_LS, JSON.stringify(positionObject))
}

function geoSuccessHandler(position) {
    const latitude = position.coords.latitude
    const longitude = position.coords.longitude
    const positionObject = {
        latitude,
        longitude
    }
    saveCoords(positionObject)
}

function geoErrorHandler() {
    console.log("Geoposition Error")
}

function askForCoords() {
    navigator.geolocation.getCurrentPosition(geoSuccessHandler, geoErrorHandler)
}

function getCoords() {
    const coords = localStorage.getItem(COORDS_LS)
    if (coords === null) {
        askForCoords()
    } else {
        //getWeather func
    }
}

function init() {
    getCoords()
}

init()