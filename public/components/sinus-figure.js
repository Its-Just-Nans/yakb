// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class SinusFigure extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: "open" });
        /* -------------------- WRAPPER -------------------- */
        const wrapper = document.createElement("div");
        wrapper.className = "web-component-sinus-figure";
        this.root.appendChild(wrapper);

        const style = document.createElement("style");
        style.textContent = `
            .web-component-sinus-figure {
                text-align: center;
            }
            .web-component-sinus-figure #canvas {
                border: 2px solid black;
                display: block;
                margin: auto;
            }
            .web-component-sinus-figure .table-wrap {
                display: inline-block;
                text-align: left;
            }
            .web-component-sinus-figure table td {
                border-bottom: 1px solid hsl(224, 10%, 23%);;
                padding: 0.5rem 1rem;
            }
        `;
        wrapper.appendChild(style);

        /* -------------------- CANVAS -------------------- */
        this.canvas = document.createElement("canvas");
        this.canvas.id = "canvas";
        this.canvas.width = 801;
        this.canvas.height = 201;
        wrapper.appendChild(this.canvas);

        /* -------------------- FORMULA TEXT -------------------- */
        this.formule = document.createElement("p");
        this.formule.id = "formule";
        wrapper.appendChild(this.formule);

        /* -------------------- TABLE WRAP -------------------- */
        const tableWrap = document.createElement("div");
        tableWrap.className = "table-wrap";
        wrapper.appendChild(tableWrap);

        /* -------------------- TABLE -------------------- */
        const table = document.createElement("table");
        const tbody = document.createElement("tbody");
        table.appendChild(tbody);
        tableWrap.appendChild(table);

        /* ---------- Helper to add slider rows ---------- */
        const makeRow = (label, id, min, max, value, step, suffix = "") => {
            const tr = document.createElement("tr");
            const td1 = document.createElement("td");
            const td2 = document.createElement("td");

            const textSpan = document.createElement("span");
            textSpan.textContent = label + ": ";

            const valueSpan = document.createElement("span");
            valueSpan.id = "text" + id;
            valueSpan.textContent = value + suffix;

            td1.appendChild(textSpan);
            td1.appendChild(valueSpan);

            const input = document.createElement("input");
            input.type = "range";
            input.id = id.toLowerCase();
            input.min = min;
            input.max = max;
            input.value = value;
            input.step = step;

            td2.appendChild(input);
            tr.appendChild(td1);
            tr.appendChild(td2);
            tbody.appendChild(tr);

            return input;
        };

        this.moy = makeRow("Mean", "Moy", -5, 5, 0, 0.1);
        this.ampli = makeRow("Amplitude", "Ampli", 0, 5, 2, 0.1);
        this.freq = makeRow("Frequency", "Freq", 0, 2, 0.5, 0.01);
        this.ret = makeRow("Phase", "Ret", 0, 2, 0, 0.1, "π");
        this.ech = makeRow("Scale", "Ech", 10, 50, 27, 1);

        this.ctx = this.canvas.getContext("2d");
    }

    connectedCallback() {
        this.ampli.addEventListener(
            "input",
            () => {
                this.root.getElementById("textAmpli").innerHTML = this.ampli.value;
                this.traceCanvas();
            },
            false
        );

        this.freq.addEventListener(
            "input",
            () => {
                this.root.getElementById("textFreq").innerHTML = this.freq.value;
                this.traceCanvas();
            },
            false
        );

        this.moy.addEventListener(
            "input",
            () => {
                this.root.getElementById("textMoy").innerHTML = this.moy.value;
                this.traceCanvas();
            },
            false
        );

        this.ret.addEventListener(
            "input",
            () => {
                this.root.getElementById("textRet").innerHTML = this.ret.value + "π";
                this.traceCanvas();
            },
            false
        );

        this.ech.addEventListener(
            "input",
            () => {
                this.Tcase = parseInt(this.ech.value, 10);
                if (Math.floor(this.canvas.height / this.Tcase) % 2 == 0) {
                    this.Ycase = (this.canvas.height - Math.floor(this.canvas.height / this.Tcase) * this.Tcase) / 2;
                } else {
                    this.Ycase =
                        (this.canvas.height - (Math.floor(this.canvas.height / this.Tcase) - 1) * this.Tcase) / 2;
                }
                if (Math.floor(this.canvas.width / this.Tcase) % 2 == 0) {
                    this.Xcase = (this.canvas.width - Math.floor(this.canvas.width / this.Tcase) * this.Tcase) / 2;
                } else {
                    this.Xcase =
                        (this.canvas.width - (Math.floor(this.canvas.width / this.Tcase) - 1) * this.Tcase) / 2;
                }
                //Xzero = Math.round(canvas.width/Tcase/2)*Tcase;
                //Yzero = Math.round(canvas.height/Tcase/2)*Tcase;
                this.root.getElementById("textEch").innerHTML = this.Tcase;
                this.traceCanvas();
            },
            false
        );
        this.init();
    }

    traceCanvas() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.fillStyle = "white";
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        this.grille(this.canvas, this.ctx);
        this.trace(this.canvas, this.ctx);
        this.formuleDisp();
    }

    init() {
        this.Tcase = parseInt(this.ech.value, 10);
        this.Xzero = this.canvas.width / 2;
        this.Yzero = this.canvas.height / 2;
        if (Math.floor(this.canvas.height / this.Tcase) % 2 == 0) {
            this.Ycase = (this.canvas.height - Math.floor(this.canvas.height / this.Tcase) * this.Tcase) / 2;
        } else {
            this.Ycase = (this.canvas.height - (Math.floor(this.canvas.height / this.Tcase) - 1) * this.Tcase) / 2;
        }

        if (Math.floor(this.canvas.width / this.Tcase) % 2 == 0) {
            this.Xcase = (this.canvas.width - Math.floor(this.canvas.width / this.Tcase) * this.Tcase) / 2;
        } else {
            this.Xcase = (this.canvas.width - (Math.floor(this.canvas.width / this.Tcase) - 1) * this.Tcase) / 2;
        }
        this.traceCanvas();
    }

    grille(canvas, inCanvas) {
        inCanvas.fillStyle = "grey";
        const height = canvas.height;
        const width = canvas.width;
        for (var i = 0 + this.Ycase; i < height; i += this.Tcase) {
            inCanvas.fillRect(0, i, width, 1);
        }
        for (var i = 0 + this.Xcase; i < width; i += this.Tcase) {
            inCanvas.fillRect(i, 0, 1, canvas.height);
        }
        inCanvas.fillStyle = "black";
        inCanvas.fillRect(this.Xzero, 0, 2, height);
        inCanvas.fillRect(0, this.Yzero, width, 2);
        inCanvas.font = "15pt Arial";
        inCanvas.fillText("x(t)", this.Xzero + 10, 18);
        inCanvas.fillText("t", width - 10, this.Yzero - 8);

        //fleches
        inCanvas.fillStyle = "black";
        inCanvas.beginPath();
        inCanvas.moveTo(this.Xzero + 1, 0);
        inCanvas.lineTo(this.Xzero + 5 + 1, 10);
        inCanvas.lineTo(this.Xzero - 5 + 1, 10);
        inCanvas.lineTo(this.Xzero + 1, 0);
        inCanvas.fill();

        inCanvas.fillStyle = "black";
        inCanvas.beginPath();
        inCanvas.moveTo(width, this.Yzero + 1);
        inCanvas.lineTo(width - 10, this.Yzero - 5 + 1);
        inCanvas.lineTo(width - 10, this.Yzero + 5 + 1);
        inCanvas.lineTo(width, this.Yzero + 1);
        inCanvas.fill();
    }

    trace(canvas, inCanvas) {
        inCanvas.moveTo(0, 100);
        inCanvas.strokeStyle = "red";
        inCanvas.beginPath();
        for (var i = 0; i < canvas.width; i++) {
            inCanvas.lineTo(
                i + 1,
                this.Yzero -
                    Math.sin(
                        2 * Math.PI * ((i + 1 - this.Xzero) / ((this.Tcase / 2) * 10)) * (this.freq.value * 5) +
                            this.ret.value * Math.PI
                    ) *
                        (this.ampli.value * this.Tcase) -
                    this.moy.value * this.Tcase
            );
        }
        inCanvas.stroke();
    }

    formuleDisp() {
        this.formule.innerHTML =
            "x(t) = " +
            this.moy.value +
            "+" +
            this.ampli.value +
            "*sin(2π*" +
            this.freq.value +
            "*t + " +
            this.ret.value +
            "π)";
    }
}

if (!customElements) {
    console.warn("sinus-figure: Web Component not registered because customElements is undefined");
} else {
    if (customElements.get("sinus-figure")) {
        console.warn("sinus-figure: Web Component not registered because sinus-figure is already defined.");
    } else {
        customElements.define("sinus-figure", SinusFigure);
    }
}
