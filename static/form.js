const json = document.getElementById("json_col");
const cbor = document.getElementById("cbor_col");

function change(type) {
    json.style = type === "json" ? "display: block;" : "display: none;";
    cbor.style = type === "cbor" ? "display: block;" : "display: none;";
}