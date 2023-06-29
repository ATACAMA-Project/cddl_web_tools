const json = document.getElementById("json");
const jsonInput = json.querySelector("textarea");
const cbor = document.getElementById("cbor");
const cborInput = cbor.querySelector("input");
const withExtra = document.querySelector("input[name='withExtra']");

function change(type) {
    withExtra.value = type;
    json.style.display = type === "json" ? "block" : "none";
    jsonInput.required = type === "json";
    cbor.style.display = type === "cbor" ? "block" : "none";
    cborInput.required = type === "cbor";

    handle = type === "codegen" ? download : submit;
}

const submitBtn = document.getElementById("submitBtn");
const loadingText = document.getElementById("loadingText");
const readyText = document.getElementById("readyText");
const results = document.getElementById("results");
const form = document.querySelector("form");

function renderJSON(alertType, title, message) {
    function escape(unsafe) {
        if (typeof unsafe !== "string") return "";

        return new Option(unsafe).innerHTML;
    }

    return "<pre class=\"alert alert-" + alertType + "\" role=\"alert\">" +
        "<h4 class=\"alert-heading\">" + escape(title) + "</h4>" +
        "<p>" + escape(message) + "</p>" +
        "</pre>";
}

function download() {
    fetch("/generate", {
        method: "POST", body: new FormData(form)
    })
        .then(async response => {
            if (response.status === 400) {
                return Promise.reject(await response.text());
            } else if (!response.ok) {
                return Promise.reject(response);
            }

            let blob = await response.blob();
            const file = window.URL.createObjectURL(blob);
            window.location.assign(file);
        })
        .catch(e => {
            if (typeof e === 'string') {
                results.innerHTML = renderJSON("warning", "Internal error:", e);
            } else if (e instanceof Response) {
                results.innerHTML = renderJSON("danger", "HTTP: " + e.status, e.statusText)
            } else {
                results.innerHTML = renderJSON("danger", "Invalid Response: ", e.message);
            }
        });
}

function submit() {
    submitBtn.disabled = true;
    readyText.style.display = "none";
    loadingText.style.display = "block";

    fetch("/validate", {
        method: "POST", body: new FormData(form)
    })
        .then(async response => {
            if (!response.ok) {
                return Promise.reject(response);
            }

            let data = await response.json();
            if (data.length === 0) {
                results.innerHTML = renderJSON("success", "Validation successful", "There are no errors in the input.");
            } else {
                results.innerHTML = data.map((d) => {
                    return renderJSON("warning", d.title, d.message);
                }).join("\n");
            }
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

handle = submit;

form.addEventListener("submit", (e) => {
    e.preventDefault();
    handle();
});

document.querySelectorAll("textarea").forEach(textArea => {
    textArea.addEventListener("keydown", function (e) {
        if (e.ctrlKey && e.key === "Enter") {
            e.preventDefault();
            handle();
        }
    });
});
