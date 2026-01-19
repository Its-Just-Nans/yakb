// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class NatoText extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });

        this.natoMap = {
            a: "Alfa",
            b: "Bravo",
            c: "Charlie",
            d: "Delta",
            e: "Echo",
            f: "Foxtrot",
            g: "Golf",
            h: "Hotel",
            i: "India",
            j: "Juliett",
            k: "Kilo",
            l: "Lima",
            m: "Mike",
            n: "November",
            o: "Oscar",
            p: "Papa",
            q: "Quebec",
            r: "Romeo",
            s: "Sierra",
            t: "Tango",
            u: "Uniform",
            v: "Victor",
            w: "Whiskey",
            x: "X-ray",
            y: "Yankee",
            z: "Zulu",

            0: "Zero",
            1: "One",
            2: "Two",
            3: "Three",
            4: "Four",
            5: "Five",
            6: "Six",
            7: "Seven",
            8: "Eight",
            9: "Niner",
        };
    }

    static get observedAttributes() {
        return ["text"];
    }

    connectedCallback() {
        const startValue = this.getAttribute("text") || "";

        this.shadowRoot.innerHTML = `
    <div class="nato-text">
        <style>
        .nato-text {
            margin: 20px 0;
        }
        textarea {
            width: 80%;
            min-height: 80px;
            font-size: 14px;
        }
        .text {
            display: flex;
            justify-content: center;
        }
        .output {
            margin-top: 8px;
            font-family: monospace;
        }
        </style>
        <div class="text">
            <textarea></textarea>
        </div>
        <div class="output"></div>
    </div>
    `;

        this.textarea = this.shadowRoot.querySelector("textarea");
        this.output = this.shadowRoot.querySelector(".output");

        this.textarea.value = startValue;
        this.updateOutput(startValue);

        this.textarea.addEventListener("input", (e) => {
            this.updateOutput(e.target.value);
        });
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "text" && this.textarea) {
            this.textarea.value = newValue || "";
            this.updateOutput(newValue || "");
        }
    }

    updateOutput(text) {
        const result = text
            .toLowerCase()
            .split("")
            .map((char, idx) => {
                if (this.natoMap[char]) return this.natoMap[char];
                if (char === " ") return " / ";
                if (char === "\n") return " / ";
                if (char === ".") {
                    if (Number.isInteger(Number(text[idx - 1])) && Number.isInteger(Number(text[idx + 1]))) {
                        return "Decimal";
                    }
                    return "Stop";
                }
                if (char === ",") return "Comma";
                if (char === "-") return "Dash";
                if (char === "/") return "Slant";
                return "";
            })
            .join(" ");

        this.output.textContent = result;
    }
}

customElements.define("nato-text", NatoText);
