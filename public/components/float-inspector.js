// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class FloatInspector extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        this.mode = "float";
        this.value = 0;
        this.bits = [];
    }

    connectedCallback() {
        this.value = Number(this.getAttribute("value")) || 0;
        this.render();
        this.setFromValue();
    }

    get format() {
        return {
            half: {
                bits: 16,
                exp: 5,
                man: 10,
                bias: 15,
            },
            bfloat: { bits: 16, exp: 8, man: 7, bias: 127 },
            float: {
                bits: 32,
                exp: 8,
                man: 23,
                bias: 127,
                stringToBits: (str) => {
                    const f32 = new Float32Array(1);
                    f32[0] = parseFloat(str);
                    const u32 = new Uint32Array(f32.buffer);
                    const bits = u32[0].toString(2).padStart(32, "0");
                    return bits;
                },
                bitsToNumber: (bits_string) => {
                    const u32 = new Uint32Array(1);
                    u32[0] = parseInt(bits_string, 2);
                    const f32 = new Float32Array(u32.buffer);
                    return f32[0];
                },
            },
            double: { bits: 64, exp: 11, man: 52, bias: 1023 },
        }[this.mode];
    }

    render() {
        this.shadowRoot.innerHTML = `
        <div class="float-inspector">
      <style>
        :host { font-family: system-ui; display:block; max-width:760px; margin:20px auto }
        .tabs { display:inline-flex; border:1px solid #ccc; border-radius:8px; overflow:hidden }
        .tab { padding:6px 14px; cursor:pointer; border-right:1px solid #ccc }
        .tab:last-child { border-right:none }
        .tab.active { background:#333; color:#fff }
        .center { text-align:center; margin:20px 0 }
        input[type=number] { font-size:32px; width:260px; text-align:center }
        input{border: 1px solid light-dark(black, white);padding: 5px;border-radius:5px;background-color: transparent;text-align: center;}
        input:focus{outline: none;}
        .bits { display:flex; flex-wrap:wrap; gap:4px; justify-content:center; font-family:monospace;color: black; }
        .bit { padding:4px 6px; border-radius:4px; cursor:pointer; user-select:none }
        .sign { background:#ffd6d6 }
        .exp  { background:#d6e4ff }
        .man  { background:#d6ffe2 }
        .row { display:flex;text-align: center; }
        .field {flex:1;}
        .field label { display: block;font-size:13px; color: light-dark(#555, #eee) }
        .field input { font-family:monospace }
        .sign-radio {flex:0.5}
      </style>

      <!-- <div class="tabs">
        ${["half", "bfloat", "float", "double"]
            .map((m) => `<div class="tab ${m === this.mode ? "active" : ""}" data-mode="${m}">${m}</div>`)
            .join("")}
      </div> -->

      <div class="center">
        <input id="valueInput" type="text" step="any">
      </div>

      <div class="bits"></div>
      <br/>
    <div class="row">
        <div class="field sign-radio">
            <label>Sign</label>
            <label>
                <input type="radio" name="sign" id="positive"> Positive
            </label>
            <label>
                <input type="radio" name="sign" id="negative"> Negative
            </label>
        </div>
        <div class="field">
            <label>Exponent</label>
            <div>
                <button class="ops" id="exp-minus">-</button>
                <input id="exp">
                <button class="ops" id="exp-plus">+</button>
            </div>
            </div>
            <div class="field">
            <label class="significand">Significand<span id="significand-frac"></span></label>
            <div>
                <button class="ops" id="man-minus">-</button>
                <input id="man">
                <button class="ops" id="man-plus">+</button>
            </div>
        </div>
        </div>
        <div class="row">
            <div class="field"><label>Calculation</label><div id="calc"></div></div>
        </div>
        </div>
    `;

        this.shadowRoot.querySelectorAll(".tab").forEach((tab) => {
            tab.onclick = () => {
                this.mode = tab.dataset.mode;
                this.render();
                this.setFromValue();
            };
        });

        this.shadowRoot.getElementById("valueInput").addEventListener("change", (e) => {
            this.value = Number(e.target.value);
            this.setFromValue();
        });

        ["positive", "negative", "exp", "man"].forEach((id) => {
            this.shadowRoot.getElementById(id).addEventListener("change", () => {
                this.setFromFields();
            });
        });

        const ops = (idElement, num) => () => {
            const el = this.shadowRoot.getElementById(idElement);
            const current = parseInt(el.value, 10);
            el.value = current + num;
            var event = new Event("change");
            el.dispatchEvent(event);
        };

        this.shadowRoot.getElementById("exp-minus").addEventListener("click", ops("exp", -1));
        this.shadowRoot.getElementById("exp-plus").addEventListener("click", ops("exp", 1));
        this.shadowRoot.getElementById("man-minus").addEventListener("click", ops("man", -1));
        this.shadowRoot.getElementById("man-plus").addEventListener("click", ops("man", 1));
    }

    setFromValue() {
        const f = this.format;
        let raw;

        if (this.mode === "float") {
            const bits = f.stringToBits(this.value);
            raw = f.bitsToNumber(bits);
        } else if (this.mode === "double") {
            const dv = new DataView(new ArrayBuffer(8));
            dv.setFloat64(0, this.value);
            raw = dv.getBigUint64(0);
        } else if (this.mode === "bfloat") {
            const dv = new DataView(new ArrayBuffer(4));
            dv.setFloat32(0, this.value);
            raw = BigInt(dv.getUint32(0) >>> 16);
        } else {
            raw = BigInt(this.floatToHalf(this.value));
        }

        this.updateUI(raw);
    }

    intToBits(n, nbBits) {
        const max = (1 << nbBits) - 1; // max value for nbBits
        if (n < 0 || n > max) {
            n = max;
        }
        return n.toString(2).padStart(nbBits, "0");
    }

    setFromFields() {
        const f = this.format;
        let bits = Array.from({ length: f.bits });
        if (this.shadowRoot.getElementById("positive").checked) {
            bits[0] = "0";
        } else {
            bits[0] = "1";
        }
        const expInt = parseInt(this.shadowRoot.getElementById("exp").value, 10) || 0;
        const exp = this.intToBits(expInt, f.exp).split("");
        const minExp = 1;
        const maxExp = 1 + f.exp;
        for (let i = minExp; i < maxExp; i++) {
            bits[i] = exp[i - minExp];
        }
        const manInt = parseInt(this.shadowRoot.getElementById("man").value, 10) || 0;
        const man = this.intToBits(manInt, f.man).split("");
        const minMan = 1 + f.exp;
        const maxMan = f.bits;
        for (let i = minMan; i < maxMan; i++) {
            bits[i] = man[i - minMan];
        }

        const raw = f.bitsToNumber(bits.join(""));

        this.updateUI(raw);
    }

    updateUI(raw) {
        const f = this.format;
        let bits = f.stringToBits(raw).split("");
        const bitsEl = this.shadowRoot.querySelector(".bits");
        bitsEl.innerHTML = "";

        bits.forEach((bit, i) => {
            const el = document.createElement("span");
            el.textContent = bit;
            el.className = "bit " + (i === 0 ? "sign" : i <= f.exp ? "exp" : "man");

            el.onclick = () => {
                bits[i] = bits[i] === "0" ? "1" : "0";
                const newNumber = f.bitsToNumber(bits.join(""));
                this.updateUI(newNumber);
            };

            bitsEl.appendChild(el);
        });

        if (bits[0] == "0") {
            this.shadowRoot.getElementById("positive").checked = true;
        } else {
            this.shadowRoot.getElementById("negative").checked = true;
        }
        const expBits = bits.slice(1, 1 + f.exp).join("");
        const exp = parseInt(expBits, 2);

        const manBits = bits.slice(1 + f.exp).join("");
        const significand = parseInt(manBits, 2);

        this.shadowRoot.getElementById("exp").value = exp;
        this.shadowRoot.getElementById("man").value = significand;

        this.shadowRoot.getElementById("valueInput").value = raw;
        const sign = bits[0] == "0" ? 1 : -1;
        const maxExponent = (1 << (f.exp - 1)) - 1;
        const frac = this.significandToFraction(manBits);
        this.shadowRoot.getElementById("significand-frac").innerText = ` = ${frac}`;
        this.shadowRoot.getElementById("calc").innerText = `${sign} × Math.pow(2, ${
            exp - maxExponent
        }) × (1 + ${frac})`;
    }

    significandToFraction(significandStr) {
        let fraction = 0;
        for (let i = 0; i < significandStr.length; i++) {
            if (significandStr[i] === "1") {
                fraction += 1 / 2 ** (i + 1); // each bit represents 1/2^(i+1)
            }
        }
        return fraction;
    }

    floatToHalf(val) {
        const dv = new DataView(new ArrayBuffer(4));
        dv.setFloat32(0, val);
        const x = dv.getUint32(0);
        const sign = x >>> 31;
        let exp = (x >>> 23) & 0xff;
        let man = x & 0x7fffff;

        if (exp === 0) return sign << 15;
        exp = exp - 127 + 15;
        man >>= 13;
        return (sign << 15) | (exp << 10) | man;
    }
}

customElements.define("float-inspector", FloatInspector);
