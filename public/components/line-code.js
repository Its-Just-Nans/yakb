class LineCode extends HTMLElement {
    constructor() {
        super();

        // Shadow DOM
        this.shadow = this.attachShadow({ mode: "open" });

        // Root container
        const container = document.createElement("div");
        container.style.textAlign = "center";

        // Styles
        const style = document.createElement("style");
        style.textContent = `
            #canvas {
                border: 1px solid black;
                margin-top: 5px;
                margin: auto;
                display: block;
            }
            #binaire {
                font-size: 0;
            }
            .buttonBinary {
                height: 40px;
                width: 40px;
                font-size: large;
                margin: 0 2px;
                display: inline-block;
            }
            .delButton, .addButton {
                padding: 10px;
            }
        `;

        // Binary buttons container
        this.binaire = document.createElement("div");
        this.binaire.id = "binaire";

        // Canvas
        this.canvas = document.createElement("canvas");
        this.canvas.id = "canvas";
        this.canvas.width = 800;
        this.canvas.height = 100;
        this.ctx = this.canvas.getContext("2d");

        // Selects
        this.motif = document.createElement("select");
        this.motif.name = "motif";
        this.motif.innerHTML = `
            <option value="nrz">Non Return to Zero (NRZ)</option>
            <option value="rz">Return to Zero (RZ)</option>
            <option value="bip">Biphase</option>
        `;

        this.format = document.createElement("select");
        this.format.name = "format";
        this.format.innerHTML = `
            <option value="unip">Unipolar</option>
            <option value="bipo">Bipolar</option>
            <option value="alte">Alternate</option>
        `;

        // Buttons
        this.addButton = document.createElement("button");
        this.addButton.className = "addButton";
        this.addButton.textContent = "Add a button";

        this.delButton = document.createElement("button");
        this.delButton.className = "delButton";
        this.delButton.textContent = "Delete a button";

        // Build DOM
        container.appendChild(this.binaire);
        container.appendChild(this.canvas);
        container.appendChild(document.createElement("br"));
        container.appendChild(this.motif);
        container.appendChild(this.format);
        container.appendChild(document.createElement("br"));
        container.appendChild(document.createElement("br"));
        container.appendChild(this.addButton);
        container.appendChild(this.delButton);

        this.shadow.appendChild(style);
        this.shadow.appendChild(container);
    }

    connectedCallback() {
        this.innerHTML = "";
        this.addButton.addEventListener("click", () => this.addButtonFn(), false);
        this.delButton.addEventListener("click", () => this.delButtonFn(), false);
        this.motif.addEventListener("input", () => this.traceCanvas(), false);
        this.format.addEventListener("input", () => this.traceCanvas(), false);

        this.init();
    }

    init() {
        if (this.canvas.getContext) {
            this.inCanvas = this.canvas.getContext("2d");
            this.traceCanvas();
        }
        for (let i = 0; i < 10; i++) {
            this.addButtonFn();
        }
    }

    getBinaryButtons() {
        return this.binaire.childNodes;
    }

    traceCanvas() {
        this.canvas.style.width = this.getBinaryButtons().length * 44 + "px";
        this.canvas.style.height = this.canvas.height + "px";
        this.Yzero = this.canvas.height / 2;
        this.Xzero = this.canvas.width / 2;
        this.Ycase = this.canvas.height / 6;
        this.Xcase = this.canvas.width / this.getBinaryButtons().length;
        this.inCanvas.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.inCanvas.fillStyle = "white";
        this.inCanvas.fillRect(0, 0, this.canvas.width, this.canvas.height);
        this.grille();
        this.trace();
    }
    trace() {
        let motif = this.motif.selectedOptions[0].value;
        let format = this.format.selectedOptions[0].value;
        this.inCanvas.strokeStyle = "red";
        this.inCanvas.lineWidth = 2;
        this.inCanvas.beginPath();
        this.inCanvas.moveTo(-1, this.Yzero);
        var compteur = 0;
        var compteurUn = 0;
        const buttons = this.getBinaryButtons();
        if (motif === "nrz") {
            if (format === "unip") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero - 2 * this.Ycase);
                    }
                }
            } else if (format === "bipo") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero + 2 * this.Ycase);
                    } else {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero - 2 * this.Ycase);
                    }
                }
            } else if (format === "alte") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        compteurUn++;
                        if (compteurUn % 2 == 0) {
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero + 2 * this.Ycase);
                            this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero + 2 * this.Ycase);
                        } else {
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                            this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero - 2 * this.Ycase);
                        }
                    }
                }
            }
        } else if (motif === "rz") {
            if (format === "unip") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    }
                }
            } else if (format === "bipo") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    }
                }
            } else if (format === "alte") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        compteurUn++;
                        if (compteurUn % 2 == 0) {
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero + 2 * this.Ycase);
                            this.inCanvas.lineTo(
                                this.Xcase * (compteur - 1) + this.Xcase / 2,
                                this.Yzero + 2 * this.Ycase
                            );
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero);
                            this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                        } else {
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                            this.inCanvas.lineTo(
                                this.Xcase * (compteur - 1) + this.Xcase / 2,
                                this.Yzero - 2 * this.Ycase
                            );
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero);
                            this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                        }
                    }
                }
            }
        } else if (motif === "bip") {
            if (format === "unip") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    }
                }
            } else if (format === "bipo") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero - 2 * this.Ycase);
                    } else {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero - 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1) + this.Xcase / 2, this.Yzero + 2 * this.Ycase);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero + 2 * this.Ycase);
                    }
                }
            } else if (format === "alte") {
                for (let item of buttons) {
                    compteur++;
                    if (item.innerHTML === "0") {
                        this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero);
                        this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero);
                    } else {
                        compteurUn++;
                        if (compteurUn % 2 == 0) {
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero + 2 * this.Ycase);
                            this.inCanvas.lineTo(
                                this.Xcase * (compteur - 1) + this.Xcase / 2,
                                this.Yzero + 2 * this.Ycase
                            );
                            this.inCanvas.lineTo(
                                this.Xcase * (compteur - 1) + this.Xcase / 2,
                                this.Yzero - 2 * this.Ycase
                            );
                            this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero - 2 * this.Ycase);
                        } else {
                            this.inCanvas.lineTo(this.Xcase * (compteur - 1), this.Yzero - 2 * this.Ycase);
                            this.inCanvas.lineTo(
                                this.Xcase * (compteur - 1) + this.Xcase / 2,
                                this.Yzero - 2 * this.Ycase
                            );
                            this.inCanvas.lineTo(
                                this.Xcase * (compteur - 1) + this.Xcase / 2,
                                this.Yzero + 2 * this.Ycase
                            );
                            this.inCanvas.lineTo(this.Xcase * compteur, this.Yzero + 2 * this.Ycase);
                        }
                    }
                }
            }
        }
        this.inCanvas.stroke();
    }

    grille() {
        this.inCanvas.fillStyle = "grey";
        this.Ycase = this.canvas.height / 6;
        this.Xcase = this.canvas.width / this.getBinaryButtons().length;
        var compteur = 0;
        for (var i = 0; i < this.canvas.height; i += this.Ycase) {
            compteur++;
            if (i == this.Ycase || i == 3 * this.Ycase || i == 5 * this.Ycase) {
                this.inCanvas.fillRect(0, Math.floor(i), this.canvas.width, 1);
            }
        }
        for (var f = 0; f < this.canvas.width - this.Xcase / 2; f += this.Xcase) {
            if (f != 0) {
                this.inCanvas.fillRect(Math.floor(f), 0, 1, this.canvas.height);
            }
        }
    }

    changeValue(valeur) {
        let buttonActual = this.shadow.getElementById("button-" + valeur);
        if (buttonActual.innerHTML == "0") {
            buttonActual.innerHTML = "1";
        } else {
            buttonActual.innerHTML = "0";
        }
    }

    addButtonFn() {
        let button = document.createElement("button");
        button.className = "buttonBinary";
        button.id = "button-" + (this.getBinaryButtons().length + 1);
        button.innerHTML = "0";
        this.binaire.append(button);
        button.addEventListener(
            "click",
            (event) => {
                const id = event.target.id;
                let numero = parseInt(id.split("-")[1], 10);
                this.changeValue(numero);
                this.traceCanvas();
            },
            false
        );
        this.traceCanvas();
    }

    delButtonFn() {
        if (this.getBinaryButtons().length >= 11) {
            this.shadow.getElementById("button-" + this.getBinaryButtons().length).remove();
        }
        this.traceCanvas();
    }
}
customElements.define("line-code", LineCode);
