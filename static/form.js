const json = document.getElementById("json");
const cbor = document.getElementById("cbor");
const cuddleRadio = document.getElementById("cuddleRadio");
const cddlRadio = document.getElementById("cddlRadio");
const typeField = document.getElementById("type");

function change(type) {
    typeField.value = type;
    json.style.display = type === "json" ? "block" : "none";
    cbor.style.display = type === "cbor" ? "block" : "none";

    cuddleRadio.disabled = type !== "";
    if (type !== "" && cuddleRadio.checked) {
        cddlRadio.checked = true;
    }
}

const form = document.querySelector("form");
const submitBtn = document.getElementById("submitBtn");
const loadingText = document.getElementById("loadingText");
const readyText = document.getElementById("readyText");
const results = document.getElementById("results");

form.addEventListener("submit", (e) => {
    e.preventDefault();

    submitBtn.disabled = true;
    readyText.style.display = "none";
    loadingText.style.display = "block";

    fetch("/validate", {
        method: "POST",
        body: new FormData(form)
    })
        .then(response => response.text())
        .then(data => {
            results.innerHTML = data;
        })
        .catch(error => {
            results.innerHTML = "<div class=\"alert alert-danger\" role=\"alert\">" + error.message + "</div>";
        })
        .finally(() => {
            submitBtn.disabled = false;
            loadingText.style.display = "none";
            readyText.style.display = "block";
        });
});