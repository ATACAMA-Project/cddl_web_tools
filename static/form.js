const json = document.getElementById("json");
const jsonInput = json.querySelector("textarea");
const cbor = document.getElementById("cbor");
const cborInput = cbor.querySelector("input");
const cuddleRadio = document.getElementById("cuddleRadio");
const cddlRadio = document.getElementById("cddlRadio");
const withExtra = document.querySelector("input[name='withExtra']");

function change(type) {
    withExtra.value = type;
    json.style.display = type === "json" ? "block" : "none";
    jsonInput.required = type === "json";
    cbor.style.display = type === "cbor" ? "block" : "none";
    cborInput.required = type === "cbor";

    cuddleRadio.disabled = type !== "plain";
    if (type !== "plain" && cuddleRadio.checked) {
        cddlRadio.checked = true;
    }
}

const submitBtn = document.getElementById("submitBtn");
const loadingText = document.getElementById("loadingText");
const readyText = document.getElementById("readyText");
const results = document.getElementById("results");
const form = document.querySelector("form");

function submit() {
    function escape(unsafe) {
        if (typeof unsafe !== "string")
            return "";

        return new Option(unsafe).innerHTML;
    }

    function renderJSON(alertType, title, message) {
        return "<pre class=\"alert alert-" + alertType + "\" role=\"alert\">" +
            "<h4 class=\"alert-heading\">" + escape(title) + "</h4>" +
            "<p>" + escape(message) + "</p>" +
            "</pre>";
    }

    submitBtn.disabled = true;
    readyText.style.display = "none";
    loadingText.style.display = "block";

    fetch("/validate", {
        method: "POST", body: new FormData(form)
    })
        .then(response => {
            if (!response.ok) {
                return Promise.reject(response);
            }

            return response.json();
        })
        .then(data => {
            results.innerHTML = renderJSON(data.alertType, data.title, data.message);
        })
        .catch(e => {
            if (e instanceof Response) {
                results.innerHTML = renderJSON("danger", "HTTP: " + e.status, e.statusText)
            } else {
                results.innerHTML = renderJSON("danger", "Invalid Response: ", e.message);
            }
        })
        .finally(() => {
            submitBtn.disabled = false;
            loadingText.style.display = "none";
            readyText.style.display = "block";
        });
}

form.addEventListener("submit", (e) => {
    e.preventDefault();
    submit();
});

document.querySelectorAll("textarea").forEach(textArea => {
    textArea.addEventListener("keydown", function (e) {
        if (e.ctrlKey && e.key === "Enter") {
            e.preventDefault();
            submit();
        }
    });
});
