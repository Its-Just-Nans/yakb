// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class CoordConverter extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
    }

    connectedCallback() {
        this.render();
        this.shadowRoot.querySelector("#convertDD").addEventListener("click", () => this.fromDD());
        this.shadowRoot.querySelector("#convertDMS").addEventListener("click", () => this.fromDMS());
        this.shadowRoot.querySelector("#convertDMM").addEventListener("click", () => this.fromDMM());
    }

    render() {
        this.shadowRoot.innerHTML = `
    <style>
      * { box-sizing: border-box; font-family: Arial, sans-serif; }
      .container { max-width: 560px; border: 1px solid #ddd; padding: 16px; }
      h3 { margin-top: 20px; }
      .row { display: flex; align-items: center; margin-bottom: 6px; }
      label { width: 90px; font-weight: bold; }
      input { padding: 6px; margin-right: 4px; }
      .small { width: 70px; }
      .calc { font-size: 12px; color: #555; margin-left: 8px; }
      button {
        margin-top: 8px;
        padding: 8px;
        width: 100%;
        background: #007bff;
        color: #fff;
        border: none;
        cursor: pointer;
      }
      .note { font-size: 12px; color: #777; margin-top: 10px; }
    </style>

    <div class="container">

      <h3>DD (Decimal Degrees)*</h3>
      <div class="row">
        <label>Latitude</label>
        <input id="ddLat" type="number" step="any" value="43.409855">
      </div>
      <div class="row">
        <label>Longitude</label>
        <input id="ddLon" type="number" step="any" value="5.230609">
      </div>
      <button id="convertDD">Convert from DD</button>

      <h3>DMS (Degrees Minutes Seconds)*</h3>
      <div class="row">
        <label>Latitude</label>
        <input id="dmsLatDeg" class="small">°
        <input id="dmsLatMin" class="small">'
        <input id="dmsLatSec" class="small">"
        <span class="calc" id="dmsLatCalc"></span>
      </div>
      <div class="row">
        <label>Longitude</label>
        <input id="dmsLonDeg" class="small">°
        <input id="dmsLonMin" class="small">'
        <input id="dmsLonSec" class="small">"
        <span class="calc" id="dmsLonCalc"></span>
      </div>
      <button id="convertDMS">Convert from DMS</button>

      <h3>DMM (Degrees Decimal Minutes)*</h3>
      <div class="row">
        <label>Latitude</label>
        <input id="dmmLatDeg" class="small">°
        <input id="dmmLatMin" class="small">'
        <span class="calc" id="dmmLatCalc"></span>
      </div>
      <div class="row">
        <label>Longitude</label>
        <input id="dmmLonDeg" class="small">°
        <input id="dmmLonMin" class="small">'
        <span class="calc" id="dmmLonCalc"></span>
      </div>
      <button id="convertDMM">Convert from DMM</button>

      <div class="note">* World Geodetic System 84 (WGS 84)</div>
    </div>
    `;
    }

    /* ---------- CORE CONVERSIONS ---------- */

    fromDD() {
        const lat = parseFloat(this.$("ddLat").value);
        const lon = parseFloat(this.$("ddLon").value);

        this.fillDMS(lat, "Lat");
        this.fillDMS(lon, "Lon");
        this.fillDMM(lat, "Lat");
        this.fillDMM(lon, "Lon");
    }

    fromDMS() {
        const lat = this.dmsToDD("Lat");
        const lon = this.dmsToDD("Lon");

        this.$("ddLat").value = lat;
        this.$("ddLon").value = lon;

        this.fillDMM(lat, "Lat");
        this.fillDMM(lon, "Lon");
    }

    fromDMM() {
        const lat = this.dmmToDD("Lat");
        const lon = this.dmmToDD("Lon");

        this.$("ddLat").value = lat;
        this.$("ddLon").value = lon;

        this.fillDMS(lat, "Lat");
        this.fillDMS(lon, "Lon");
    }

    /* ---------- HELPERS ---------- */

    fillDMS(value, axis) {
        const deg = Math.trunc(value);
        const minFloat = Math.abs((value - deg) * 60);
        const min = Math.trunc(minFloat);
        const sec = ((minFloat - min) * 60).toFixed(3);

        this.$(`dms${axis}Deg`).value = deg;
        this.$(`dms${axis}Min`).value = min;
        this.$(`dms${axis}Sec`).value = sec;

        this.$(`dms${axis}Calc`).textContent = `(${value} − ${deg}) × 60 → ${min}' ${sec}"`;
    }

    fillDMM(value, axis) {
        const deg = Math.trunc(value);
        const min = Math.abs((value - deg) * 60).toFixed(3);

        this.$(`dmm${axis}Deg`).value = deg;
        this.$(`dmm${axis}Min`).value = min;

        this.$(`dmm${axis}Calc`).textContent = `(${value} − ${deg}) × 60 = ${min}'`;
    }

    dmsToDD(axis) {
        const d = parseFloat(this.$(`dms${axis}Deg`).value);
        const m = parseFloat(this.$(`dms${axis}Min`).value);
        const s = parseFloat(this.$(`dms${axis}Sec`).value);

        const dd = (Math.abs(d) + m / 60 + s / 3600) * Math.sign(d || 1);
        this.$(`dms${axis}Calc`).textContent = `${d} + ${m}/60 + ${s}/3600 = ${dd.toFixed(6)}`;
        return dd.toFixed(6);
    }

    dmmToDD(axis) {
        const d = parseFloat(this.$(`dmm${axis}Deg`).value);
        const m = parseFloat(this.$(`dmm${axis}Min`).value);

        const dd = (Math.abs(d) + m / 60) * Math.sign(d || 1);
        this.$(`dmm${axis}Calc`).textContent = `${d} + ${m}/60 = ${dd.toFixed(6)}`;
        return dd.toFixed(6);
    }

    $(id) {
        return this.shadowRoot.getElementById(id);
    }
}

customElements.define("coord-converter", CoordConverter);
