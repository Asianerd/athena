function sidebar_setState(state) {
    let d = document.querySelector("#overlay #sidebar");
    d.setAttribute("state", state);
}

function sidebar_toggleTeamSelection() {
    let d = document.querySelector("#sidebar #projects #teams #choices");
    if (d.attributes.getNamedItem("state") == null) {
        d.setAttribute("state", "open");
    } else {
        d.setAttribute("state", d.attributes.getNamedItem("state").nodeValue == "open" ? "closed" : "open");
    }
}

sendPostRequest(`${ATHENA_BACKEND_ADDRESS}/user/fetch_teams`, login_info(), (r) => {
    let response = JSON.parse(parseResponse(r));
    console.log(response);
})

document.querySelector("#sidebar #self #username").innerHTML = username;
