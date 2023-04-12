const json = document.getElementById("json");
const cbor = document.getElementById("cbor");
const cuddleRadio = document.getElementById("cuddleRadio");
const cddlRadio = document.getElementById("cddlRadio");
const typeField = document.getElementById("type");

function change(type) {
    typeField.value = type;
    json.style.display = type === "json" ? "block" : "none";
    cbor.style.display = type === "cbor" ? "block" : "none";

    if (type === "json") {
        cuddleRadio.disabled = true;
        if (cddlRadio.checked) {
            cuddleRadio.checked = true;
        }
    }
}

const form = document.getElementById("form");
const submitBtn = document.getElementById("submitBtn");
const loadingText = document.getElementById("loadingText");
const readyText = document.getElementById("readyText");
const results = document.getElementById("results");

form.addEventListener("submit", (e) => {
    e.preventDefault();
    const formData = new FormData(form);

    submitBtn.disabled = true;
    readyText.style.display = "none";
    loadingText.style.display = "block";

    fetch("/validate", {
        method: "POST",
        body: formData
    })
        .then(response => response.text())
        .then(data => {
            results.innerHTML = data;
            submitBtn.disabled = false;
            loadingText.style.display = "none";
            readyText.style.display = "block";
        })
        .catch(error => {
            results.innerHTML = "<div className=\"alert alert-warning\" role=\"alert\">" + error.message + "</div>";
            submitBtn.disabled = false;
            loadingText.style.display = "none";
            readyText.style.display = "block";
        });
});