// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class ChmodSelector extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
    }

    connectedCallback() {
        this.render();
        this.shadowRoot.addEventListener("change", () => this.update());
        this.update();
    }

    render() {
        this.shadowRoot.innerHTML = `
<div class="chmod-selector">
    <style>
    .chmod-selector {
        margin: 20px 0;
    }
    table {
        border-collapse: collapse;
        font-family: monospace;
    }
    th, td {
        border: 1px solid #444;
        padding: 6px 10px;
        text-align: center;
    }
    th.group {
        font-weight: bold;
    }
    .output {
        flex: 1;
        margin-top: 12px;
        font-family: monospace;
        text-align: center;
    }
    </style>
    <table>
    <tr>
        <th></th>
        <th colspan="3">Special bits</th>
        <th class="group" colspan="3">User</th>
        <th class="group" colspan="3">Group</th>
        <th class="group" colspan="3">Other</th>
    </tr>
    <tr>
        <th></th>
        <td>setuid</td>
        <td>setgid</td>
        <td>sticky</td>
        <th>r (4) </th><th>w (2) </th><th>x (1) </th>
        <th>r (4) </th><th>w (2) </th><th>x (1) </th>
        <th>r (4) </th><th>w (2) </th><th>x (1) </th>
    </tr>
    <tr>
        <th>Permissions</th>
        <td><input type="checkbox" data-special="4"></td>
        <td><input type="checkbox" data-special="2"></td>
        <td><input type="checkbox" data-special="1"></td>
        ${this.permCells("u")}
        ${this.permCells("g")}
        ${this.permCells("o")}
    </tr>
    </table>
    <div class="output">
        <div id="numeric"></div>
        <div id="symbolic"></div>
    </div>
</div>
    `;
    }

    permCells(scope) {
        return ["r", "w", "x"]
            .map((p, i) => `<td><input type="checkbox" data-scope="${scope}" data-bit="${4 >> i}"></td>`)
            .join("");
    }

    update() {
        const scopes = { u: 0, g: 0, o: 0 };

        this.shadowRoot.querySelectorAll("input[data-scope]").forEach((cb) => {
            if (cb.checked) {
                scopes[cb.dataset.scope] += Number(cb.dataset.bit);
            }
        });

        let special = 0;
        this.shadowRoot.querySelectorAll("input[data-special]").forEach((cb) => {
            if (cb.checked) {
                special += Number(cb.dataset.special);
            }
        });

        const numeric = `${special}${scopes.u}${scopes.g}${scopes.o}`;

        const symbolic = Object.entries(scopes)
            .map(([scope, val]) => {
                const perms = (val & 4 ? "r" : "") + (val & 2 ? "w" : "") + (val & 1 ? "x" : "");
                return perms ? `${scope}+${perms}` : "";
            })
            .filter(Boolean)
            .join(",");

        this.shadowRoot.getElementById("numeric").textContent = `chmod ${numeric}`;

        this.shadowRoot.getElementById("symbolic").textContent = symbolic ? `chmod ${symbolic}` : "";
    }
}

customElements.define("chmod-selector", ChmodSelector);
