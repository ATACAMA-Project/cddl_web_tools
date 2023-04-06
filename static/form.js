const json = document.getElementById("json");
const cbor = document.getElementById("cbor");
const cuddleRadio = document.getElementById("cuddleRadio");
const cddlRadio = document.getElementById("cddlRadio");
const typeField = document.getElementById("type");

function change(type) {
    typeField.value = type;
    json.style.display = type === "json" ? "block" : "none";
    cbor.style.display = type === "cbor" ? "block" : "none";
    cuddleRadio.disabled = type === "json";
    if (type === "json" && cuddleRadio.checked) {
        cddlRadio.checked = true;
    }
}