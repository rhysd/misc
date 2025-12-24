const ONGOING_INIT = { weapon: null, element: null, count: null };
function createTH(text, className) {
    const th = document.createElement('th');
    th.textContent = text;
    if (className) {
        th.className = className;
    }
    return th;
}
class App {
    addButton;
    table;
    ongoing;
    doneCounts;
    constructor() {
        this.addButton = document.getElementById('add');
        this.addButton.addEventListener('click', this.onAddButtonClicked.bind(this));
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
        this.addButton.disabled = !(this.ongoing.weapon && this.ongoing.element && this.ongoing.count);
        const { weapon, element } = this.ongoing;
        if (weapon && element) {
            const count = this.doneCounts.get(weapon).get(element);
            this.disableCountUntil(count);
        }
    }
    onAddButtonClicked(event) {
        if (this.addButton.disabled) {
            return;
        }
        const { weapon, element, count } = this.ongoing;
        const tr = document.createElement('tr');
        tr.appendChild(createTH(count.toString(), 'found-count'));
        tr.appendChild(createTH(weapon));
        tr.appendChild(createTH(element));
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
        this.update();
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