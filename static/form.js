const json = document.getElementById("json");
const cbor = document.getElementById("cbor");

function change(type) {
    json.style = type === "json" ? "display: block;" : "display: none;";
    cbor.style = type === "cbor" ? "display: block;" : "display: none;";
}