const toPercentage = (value) => {
    if (typeof value !== "number") {
        return value;
    }

    var prepend = "";
    if(value) {
        prepend = value > 0 ? "▲" : "▼"; 
    }

    return prepend+value+"%";
}

export default toPercentage;