const ONGOING_INIT = { weapon: null, element: null, count: null };
function createTH(child, className) {
    const th = document.createElement('th');
    if (typeof child === 'string') {
        th.textContent = child;
    }
    else {
        th.appendChild(child);
    }
    if (className) {
        th.className = className;
    }
    return th;
}
class App {
    table;
    ongoing;
    doneCounts;
    constructor() {
        this.table = document.getElementById('candidates');
        this.doneCounts = new Map();
        this.ongoing = { ...ONGOING_INIT };
        for (const span of document.querySelectorAll('#select-weapon .item')) {
            const name = span.querySelector('label').textContent;
            const input = span.querySelector('input');
            input.addEventListener('change', this.onWeaponClicked.bind(this, name));
            this.doneCounts.set(name, new Map());
        }
        for (const span of document.querySelectorAll('#select-element .item')) {
            const name = span.querySelector('label').textContent;
            const input = span.querySelector('input');
            input.addEventListener('change', this.onElementClicked.bind(this, name));
            for (const m of this.doneCounts.values()) {
                m.set(name, 0);
            }
        }
        for (const span of document.querySelectorAll('#select-count .item')) {
            const count = parseInt(span.querySelector('label').textContent, 10);
            const input = span.querySelector('input');
            input.addEventListener('change', this.onCountClicked.bind(this, count));
        }
        const resetButton = document.getElementById('reset');
        resetButton.addEventListener('click', this.reset.bind(this));
    }
    onWeaponClicked(name, event) {
        const input = event.target;
        if (!input.checked) {
            return;
        }
        this.ongoing.weapon = name;
        this.update();
    }
    onElementClicked(name, event) {
        const input = event.target;
        if (!input.checked) {
            return;
        }
        this.ongoing.element = name;
        this.update();
    }
    onCountClicked(count, event) {
        const input = event.target;
        if (!input.checked) {
            return;
        }
        this.ongoing.count = count;
        this.update();
    }
    update() {
        const { weapon, element, count } = this.ongoing;
        if (weapon && element) {
            const count = this.doneCounts.get(weapon).get(element);
            this.disableCountUntil(count);
        }
        if (weapon === null || element === null || count === null) {
            return;
        }
        const tr = document.createElement('tr');
        tr.appendChild(createTH(count.toString(), 'found-count'));
        tr.appendChild(createTH(weapon, 'found-weapon'));
        tr.appendChild(createTH(element, 'found-element'));
        const close = document.createElement('button');
        close.className = 'close';
        close.addEventListener('click', this.deleteCandidate.bind(this, weapon, element));
        tr.appendChild(createTH(close));
        const n = this.findCandiatePosition(count);
        if (n === null) {
            this.table.appendChild(tr);
        }
        else {
            this.table.insertBefore(tr, n);
        }
        this.disableCountUntil(count);
        this.doneCounts.get(weapon).set(element, count);
        this.ongoing.count = null;
    }
    findCandiatePosition(count) {
        for (const n of this.table.children) {
            const c = parseInt(n.querySelector('.found-count').textContent, 10);
            if (c >= count) {
                return n;
            }
        }
        return null;
    }
    disableCountUntil(count) {
        const elems = document.querySelectorAll('#select-count input');
        for (let i = 0; i < elems.length; i++) {
            const elem = elems[i];
            elem.disabled = (i + 1) <= count;
            if (elem.disabled && elem.checked) {
                elem.checked = false;
            }
        }
    }
    deleteCandidate(weapon, element) {
        this.doneCounts.get(weapon).set(element, 0);
        this.update();
        for (const row of this.table.children) {
            const w = row.querySelector('.found-weapon')?.textContent;
            const e = row.querySelector('.found-element')?.textContent;
            if (w === weapon && e === element) {
                this.table.removeChild(row);
                return;
            }
        }
    }
    reset() {
        this.ongoing = { ...ONGOING_INIT };
        this.disableCountUntil(0);
        for (const m of this.doneCounts.values()) {
            for (const k of m.keys()) {
                m.set(k, 0);
            }
        }
        const checked = document.querySelectorAll('input[type="radio"]:checked');
        for (const input of checked) {
            input.checked = false;
        }
        this.table.replaceChildren();
    }
}
new App();
export {};
//# sourceMappingURL=index.js.map