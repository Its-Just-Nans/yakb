// This file is part of https://github.com/Its-Just-Nans/yakb
// It is licensed under the same license of https://github.com/Its-Just-Nans/yakb

class IPv4Calculator extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        this.famousIPs = {
            "8.8.8.8": "Google DNS",
            "8.8.4.4": "Google DNS",
            "1.1.1.1": "Cloudflare DNS",
            "1.0.0.1": "Cloudflare DNS",
            "9.9.9.9": "Quad9 DNS",
        };
        this.shadowRoot.innerHTML = `
    <div class="calculator">
    <style>
    .calculator {
        display: block;
        font-family: Arial, sans-serif;
        border: 1px solid #ccc;
        padding: 10px;
    }
    label {
        display: block;
        margin-top: 5px;
    }
    input {
        width: 100%;
        box-sizing: border-box;
        margin-top: 3px;
        padding: 5px;
    }
    .output {
        background: #f9f9f9;
        margin-top: 10px;
        padding: 10px;
        border-radius: 5px;
        color: black;
        font-family: monospace;
    }
    .private {
        color: green;
        font-weight: bold;
    }
    .binary {
        font-family: monospace;
        background: #eee;
        padding: 3px;
        border-radius: 3px;
        display: inline-block;
        margin-top: 2px;
    }
    .span-cidr {
        color: red;
    }
    </style>
        <label>IPv4 Address:
        <input type="text" id="ip" placeholder="e.g. 192.168.0.1">
        </label>
        <label>Mask <span class="span-cidr">(CIDR)</span>:
        <input type="number" id="cidr" min="0" max="32" placeholder="e.g. 24">
        </label>
        <label>Mask (255 style):
        <input type="text" id="mask" placeholder="e.g. 255.255.255.0">
        </label>
        <div class="output" id="outputBinary"></div>
        <div class="output" id="output"></div>
    </div>
    `;
        this.ipInput = this.shadowRoot.getElementById("ip");
        this.cidrInput = this.shadowRoot.getElementById("cidr");
        this.maskInput = this.shadowRoot.getElementById("mask");
        this.outputDiv = this.shadowRoot.getElementById("output");
        this.outputBinaryDiv = this.shadowRoot.getElementById("outputBinary");
    }

    connectedCallback() {
        this.ipInput.addEventListener("input", () => this.update());
        this.cidrInput.addEventListener("input", () => this.updateFromCIDR());
        this.maskInput.addEventListener("input", () => this.updateFromMask());
        this.ipInput.value = this.getAttribute("ip");
        this.maskInput.value = this.getAttribute("cidr");
        this.update();
    }

    updateFromCIDR() {
        const cidr = parseInt(this.cidrInput.value);
        if (cidr >= 0 && cidr <= 32) {
            this.maskInput.value = this.cidrToMask(cidr);
        }
        this.update();
    }

    updateFromMask() {
        const mask = this.maskInput.value.trim();
        const cidr = this.maskToCidr(mask);
        if (cidr !== null) {
            this.cidrInput.value = cidr;
        }
        this.update();
    }

    update() {
        const ip = this.ipInput.value.trim();
        const mask = this.maskInput.value.trim();
        const cidr = this.maskToCidr(mask);
        if (!this.cidrInput.value) {
            this.cidrInput.value = cidr;
        }
        if (!this.validateIP(ip)) {
            this.outputBinaryDiv.innerHTML = "";
            this.outputDiv.innerHTML = "Invalid IP";
            return;
        }
        if (!this.validateMask(mask)) {
            this.outputBinaryDiv.innerHTML = "";
            this.outputDiv.innerHTML = "Invalid mask";
            return;
        }

        const ipNum = this.ipToNum(ip);
        const maskNum = this.ipToNum(mask);
        const network = ipNum & maskNum;
        const broadcast = network | (~maskNum >>> 0);
        const first = network + (cidr >= 31 ? 0 : 1);
        const last = broadcast - (cidr >= 31 ? 0 : 1);
        const usable =
            cidr === 31
                ? `2 (<a href="https://datatracker.ietf.org/doc/html/rfc3021" style="color: blue;">RFC 3021</a>)`
                : cidr === 32
                ? "1"
                : `${last - first + 1}`;

        const ipBin = this.toBinary(ipNum);
        const maskBin = this.toBinary(maskNum);
        const networkBin = this.toBinary(network);
        const broadcastBin = this.toBinary(broadcast);
        const firstBin = this.toBinary(first);
        const lastBin = this.toBinary(last);
        const famousText = this.famousIPs[ip] ? ` - ${this.famousIPs[ip]}` : "";
        const ipText = this.isPrivate(ip) ? '<span class="private">Private IP</span>' : `Public IP${famousText}`;

        const coloredMask = this.highlightSubnet(maskBin, cidr);
        const coloredNetwork = this.highlightSubnet(networkBin, cidr);

        this.outputBinaryDiv.innerHTML = `
        <div class="binary">${ipBin}</div> = ${ip} = IPv4 (${ipText})
        <br/>
        <div class="binary">${coloredMask}</div> = ${mask} = Mask (<span style="color:red">${cidr} one</span>)
        <br/>
        <div class="binary">${coloredNetwork}</div> = ${this.numToIp(network)} = Network
        `;

        this.outputDiv.innerHTML = `
      <div>First Usable: ${this.numToIp(first)} <div class="binary">${this.highlightSubnet(firstBin, cidr)}</div></div>
      <div>Last Usable: ${this.numToIp(last)} <div class="binary">${this.highlightSubnet(lastBin, cidr)}</div></div>
      <div>Broadcast: ${this.numToIp(broadcast)} <div class="binary">${this.highlightSubnet(
            broadcastBin,
            cidr
        )}</div></div>
      <div>Usable Hosts: ${usable}</div>
    `;
    }

    highlightSubnet(binaryMask, cidr) {
        let count = 0;
        let result = "";

        for (let char of binaryMask) {
            if (char === "1" || char === "0") {
                // Highlight network bits
                if (count < cidr) {
                    result += `<span style="color:red">${char}</span>`;
                } else {
                    result += char;
                }
                count++;
            } else {
                // Preserve dots or any separators
                result += char;
            }
        }

        return result;
    }

    validateIP(ip) {
        return /^(\d{1,3}\.){3}\d{1,3}$/.test(ip) && ip.split(".").every((n) => n >= 0 && n <= 255);
    }

    validateMask(mask) {
        const validOctets = [0, 128, 192, 224, 240, 248, 252, 254, 255];
        const nums = mask.split(".");
        if (nums.some((n) => n.length === 0)) {
            return false;
        }
        const num = nums.map(Number);
        if (num.length !== 4 || num.some((n) => !validOctets.includes(n))) return false;
        // Additionally, ensure overall mask is contiguous 1s followed by 0s
        const bin = num.map((n) => n.toString(2).padStart(8, "0")).join("");
        return /^1*0*$/.test(bin);
    }

    ipToNum(ip) {
        return ip.split(".").reduce((acc, n) => (acc << 8) + parseInt(n), 0) >>> 0;
    }

    numToIp(num) {
        return [(num >>> 24) & 0xff, (num >>> 16) & 0xff, (num >>> 8) & 0xff, num & 0xff].join(".");
    }

    cidrToMask(cidr) {
        const maskNum = cidr === 0 ? 0 : (~0 << (32 - cidr)) >>> 0;
        return this.numToIp(maskNum);
    }

    maskToCidr(mask) {
        if (!this.validateMask(mask)) return null;
        const bin = mask
            .split(".")
            .map((n) => parseInt(n).toString(2).padStart(8, "0"))
            .join("");
        return bin.indexOf("0") === -1 ? 32 : bin.indexOf("0");
    }

    isPrivate(ip) {
        const parts = ip.split(".").map(Number);
        return (
            parts[0] === 10 ||
            (parts[0] === 172 && parts[1] >= 16 && parts[1] <= 31) ||
            (parts[0] === 192 && parts[1] === 168)
        );
    }

    toBinary(num) {
        return [
            ((num >>> 24) & 0xff).toString(2).padStart(8, "0"),
            ((num >>> 16) & 0xff).toString(2).padStart(8, "0"),
            ((num >>> 8) & 0xff).toString(2).padStart(8, "0"),
            (num & 0xff).toString(2).padStart(8, "0"),
        ].join(".");
    }
}

customElements.define("ipv4-calculator", IPv4Calculator);
