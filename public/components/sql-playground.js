// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class SqlPlayground extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: "open" });
        /* -------------------- WRAPPER -------------------- */
        const wrapper = document.createElement("div");
        wrapper.className = "web-component-sql-playground";
        this.root.appendChild(wrapper);

        this.textarea = document.createElement("textarea");
        this.textarea.rows = 10;
        this.textarea.cols = 50;
        this.textarea.placeholder = "Enter your SQL query here...";
        wrapper.appendChild(this.textarea);

        this.result = document.createElement("pre");
        wrapper.appendChild(this.result);

        this.buttonExec = document.createElement("button");
        this.buttonExec.innerText = "Execute";
        wrapper.appendChild(this.buttonExec);

        this.buttonExec.disabled = true;
        this.result.innerText = "Error initializing SQL.js. See console for details.";
    }

    connectedCallback() {
        this.initAsync();

        this.buttonExec.addEventListener(
            "click",
            () => {
                console.log("Executing SQL...");
                this.execSQL();
            },
            false
        );
    }

    execSQL() {
        const query = this.textarea.value;
        const res = this.db.exec(query);
        this.result.innerText = "";
        setTimeout(() => {
            this.result.innerText = JSON.stringify(res, null, 4);
        }, 250);
    }

    async initAsync() {
        await new Promise((resolve, reject) => {
            const s = document.createElement("script");
            s.src = "https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.13.0/sql-wasm.js";
            s.onload = resolve;
            s.onerror = reject;
            document.head.appendChild(s);
        });
        const initSqlJs = window.initSqlJs;
        const SQL = await initSqlJs({
            // Required to load the wasm binary asynchronously. Of course, you can host it wherever you want
            // You can omit locateFile completely when running in node
            locateFile: (file) => `https://sql.js.org/dist/${file}`,
        });
        this.db = new SQL.Database();
        this.buttonExec.disabled = false;
        this.result.innerText = "SQL.js initialized. You can now execute queries.";
    }
}

customElements.define("sql-playground", SqlPlayground);
