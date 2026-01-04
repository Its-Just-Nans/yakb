// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class TextBase64Converter extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = `
    <div>
    <style>
        textarea {
            width: 90%;
            min-height: 200px;
            margin: 8px 0;
            font-family: monospace;
        }
        label {
            font-weight: bold;
            display: block;
        }
        .converter {
            display: flex;
            flex-direction: row;
        }
        .side {
            flex: 1;
        }
        @media only screen and (max-width: 600px) {
            .converter {
                flex-direction: column;
            }
        }
    </style>
        <div class="converter">
        <div class="side">
        <label>Text</label>
        <textarea id="text"></textarea>
        </div>
        <div class="side">
        <label>Base64</label>
        <textarea id="base64"></textarea>
        </div>
        </div>
    </div>`;
    }

    connectedCallback() {
        const textArea = this.shadowRoot.getElementById("text");
        const base64Area = this.shadowRoot.getElementById("base64");

        let isUpdating = false;

        textArea.addEventListener("input", () => {
            if (isUpdating) return;
            isUpdating = true;

            try {
                base64Area.value = btoa(unescape(encodeURIComponent(textArea.value)));
            } catch {
                base64Area.value = "";
            }

            isUpdating = false;
        });

        base64Area.addEventListener("input", () => {
            if (isUpdating) return;
            isUpdating = true;

            try {
                textArea.value = decodeURIComponent(escape(atob(base64Area.value)));
            } catch {
                textArea.value = "";
            }

            isUpdating = false;
        });
    }
}

customElements.define("text-base64-converter", TextBase64Converter);
