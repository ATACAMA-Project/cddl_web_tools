const json = document.getElementById("json");
const jsonInput = json.querySelector("textarea");
const cbor = document.getElementById("cbor");
const cborInput = cbor.querySelector("input");
const withExtra = document.querySelector("input[name='withExtra']");
const readyText = document.getElementById("readyText");

// handles the display changes when a tab is clicked
function change(type) {
    withExtra.value = type;
    json.style.display = type === "json" ? "block" : "none";
    jsonInput.required = type === "json";
    cbor.style.display = type === "cbor" ? "block" : "none";
    cborInput.required = type === "cbor";

    readyText.innerHTML = type === "codegen" ? "Generate" : "Validate";
    handle = type === "codegen" ? download : submit;
}

const submitBtn = document.getElementById("submitBtn");
const loadingText = document.getElementById("loadingText");
const results = document.getElementById("results");
const form = document.querySelector("form");

// renders the json into a bootstrap alert
function renderJSON(alertType, title, message) {
    // escapes the html
    function escape(unsafe) {
        if (typeof unsafe !== "string") return "";

        return new Option(unsafe).innerHTML;
    }

    return `<pre class="alert alert-${alertType}" role="alert"><h4 class="alert-heading">${escape(title)}</h4><p>${escape(message)}</p></pre>`;
}

// downloads code
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

            // get the file, create an Object url, create a link which targets the url and clicks it
            const blob = await response.blob();
            const file = window.URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = file;
            a.download = "generated_code.zip";
            a.click();
            window.URL.revokeObjectURL(file);
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

// submits cddl for validation
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

            const data = await response.json();
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

// saves what happens when submit is clicked
let handle = submit;

// handles the form submission via button click
form.addEventListener("submit", (e) => {
    // makes sure form is not submitted and web page is not reloaded
    e.preventDefault();

    handle();
});

// handles the form submission via ctrl+enter
document.querySelectorAll("textarea").forEach(textArea => {
    textArea.addEventListener("keydown", function (e) {
        if (e.ctrlKey && e.key === "Enter") {
            e.preventDefault();
            handle();
        }
    });
});
