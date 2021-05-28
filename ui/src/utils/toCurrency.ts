const toCurrency = (value, cashflow=false) => {
    if (typeof value !== "number") {
        return value;
    }

    var prepend = "";
    if(cashflow) {
        prepend = value > 0 ? "+" : ""; 
    }

    var formatter = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: 0
    });
    return prepend+formatter.format(value);
}

export default toCurrency;