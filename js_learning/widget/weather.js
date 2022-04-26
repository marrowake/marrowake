const COORDS_LS = 'coords'
const API_KEY = 'e853ca1f327cc45d83d2598f9d475f58' //https://home.openweathermap.org/api_keys password=NA7JrVbF

const weatherContainer = document.querySelector('.js-weather')

function getWeather(lat, lng) {
    fetch(`https://api.openweathermap.org/data/2.5/weather?lat=${lat}&lon=${lng}&appid=${API_KEY}&units=metric`)
        .then(function(response) {
            return response.json()
        })
        .then(function(json) {
            const temperature = json.main.temp
            const place = json.name
            weatherContainer.innerText = `${temperature} @ ${place}`
        })
}

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
    getWeather(latitude, longitude)
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
        const loadedCoords = JSON.parse(coords)
        getWeather(loadedCoords.latitude, loadedCoords.longitude)
    }
}

function init() {
    getCoords()
}

init()