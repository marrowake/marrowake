const calculator = {
    plus: function(a, b) {
        return a + b
    },
    minus: function(a, b) {
        return a - b
    },
    multiply: function(a, b) {
        return a * b
    },
    divide: function(a, b) {
        return a / b
    },
    upSqr: function(a, b) {
        return a ** b
    }
}

const summ = calculator.plus(13, 8)
const diff = calculator.minus(24, 5)
const multp = calculator.multiply(3, 6)
const divd = calculator.divide(25, 5)
const sqr = calculator.upSqr(2, 4)

console.log(summ)
console.log(diff)
console.log(multp)
console.log(divd)
console.log(sqr)