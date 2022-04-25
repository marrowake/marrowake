const toDoForm = document.querySelector('.js-toDoForm'),
    toDoInput = toDoForm.querySelector('input'),
    toDoList = document.querySelector('.js-toDoList')

const TODOES_LS = 'toDoes'
const toDoesArr = []

function foo(toDo) {
    showToDoes(toDo.name)
}

function loadToDoes() {
    const loaded_toDoes = localStorage.getItem(TODOES_LS)
    if (loaded_toDoes !== null) {
        const parsedToDoes = JSON.parse(loaded_toDoes)
        parsedToDoes.forEach(foo)
    }
}

function saveToDoes() {
    localStorage.setItem(TODOES_LS, JSON.stringify(toDoesArr)) //JSON.stringify converts objsect into a string
}

function showToDoes(text) {
    const li = document.createElement('li')
    const delBtn = document.createElement('button')
    const span = document.createElement('span')
    const newId = toDoesArr.length + 1
    delBtn.innerHTML = '&#10060'
    span.innerText = text
    li.appendChild(delBtn)
    li.appendChild(span)
    li.id = newId
    toDoList.appendChild(li)
    const toDoObject = {
        name: text,
        id: newId
    }
    toDoesArr.push(toDoObject)
    saveToDoes()
}

function submitHandler(event) {
    event.preventDefault()
    const currentValue = toDoInput.value
    showToDoes(currentValue)
    toDoInput.value = ""
}

function init() {
    loadToDoes()
    toDoForm.addEventListener('submit', submitHandler)
}

init()