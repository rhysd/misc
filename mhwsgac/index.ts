interface WeaponItem {
    name: string;
    input: HTMLInputElement;
}

interface Ongoing {
    weapon: string | null;
    element: string | null;
    count: number | null;
}

const ONGOING_INIT: Ongoing = { weapon: null, element: null, count: null };

function createTH(child: string | HTMLElement, className?: string): HTMLTableCellElement {
    const th = document.createElement('th');
    if (typeof child === 'string') {
        th.textContent = child;
    } else {
        th.appendChild(child);
    }
    if (className) {
        th.className = className;
    }
    return th;
}

class App {
    table: HTMLElement;
    tableRoot: HTMLElement;
    countsRoot: HTMLElement;
    ongoing: Ongoing;
    doneCounts: Map<string, Map<string, number>>;

    constructor() {
        this.tableRoot = document.getElementById('table-root')!;
        this.table = document.getElementById('table-body')!;
        this.countsRoot = document.getElementById('select-count')!;
        this.doneCounts = new Map();
        this.ongoing = { ...ONGOING_INIT };

        for (const span of document.querySelectorAll('#select-weapon .item')) {
            const name = span.querySelector('label')!.textContent;
            const input = span.querySelector('input')! as HTMLInputElement;
            input.addEventListener('change', this.onWeaponClicked.bind(this, name));
            this.doneCounts.set(name, new Map());
        }
        for (const span of document.querySelectorAll('#select-element .item')) {
            const name = span.querySelector('label')!.textContent;
            const input = span.querySelector('input')! as HTMLInputElement;
            input.addEventListener('change', this.onElementClicked.bind(this, name));
            for (const m of this.doneCounts.values()) {
                m.set(name, 0);
            }
        }
        this.prepareCounts(10);

        const resetButton = document.getElementById('reset-button')! as HTMLButtonElement;
        resetButton.addEventListener('click', this.reset.bind(this));

        const configDialog = document.getElementById('config-dialog')! as HTMLDialogElement;
        const configButton = document.getElementById('config-button')! as HTMLButtonElement;
        configButton.addEventListener('click', () => {
            configDialog.open = !configDialog.open;
        });
        const configMaxCount = document.getElementById('config-max-count')! as HTMLInputElement;
        document.getElementById('dialog-close')!.addEventListener('click', () => {
            configDialog.open = false;
            this.prepareCounts(parseInt(configMaxCount.value, 10));
            this.reset();
        });
    }

    onWeaponClicked(name: string, event: Event): void {
        const input = event.target! as HTMLInputElement;
        if (!input.checked) {
            return;
        }
        this.ongoing.weapon = name;
        this.update();
    }

    onElementClicked(name: string, event: Event): void {
        const input = event.target! as HTMLInputElement;
        if (!input.checked) {
            return;
        }
        this.ongoing.element = name;
        this.update();
    }

    onCountClicked(count: number, event: Event): void {
        const input = event.target! as HTMLInputElement;
        if (!input.checked) {
            return;
        }
        this.ongoing.count = count;
        this.update();
    }

    update(): void {
        const { weapon, element, count } = this.ongoing;
        if (weapon && element) {
            const count = this.doneCounts.get(weapon)!.get(element)!;
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
        close.className = 'delete-row';
        close.addEventListener('click', this.deleteRow.bind(this, weapon, element));
        tr.appendChild(createTH(close));
        const n = this.findCandiatePosition(count);
        if (n === null) {
            this.table.appendChild(tr);
        } else {
            this.table.insertBefore(tr, n);
        }
        this.tableRoot.classList.remove('hidden');

        this.disableCountUntil(count);
        this.doneCounts.get(weapon)!.set(element, count);
        this.ongoing.count = null;
    }

    findCandiatePosition(count: number): Node | null {
        for (const n of this.table.children) {
            const c = parseInt(n.querySelector('.found-count')!.textContent, 10);
            if (c >= count) {
                return n;
            }
        }
        return null;
    }

    disableCountUntil(count: number): void {
        const elems = document.querySelectorAll('#select-count input') as NodeListOf<HTMLInputElement>;
        for (let i = 0; i < elems.length; i++) {
            const elem = elems[i]!;
            elem.disabled = i + 1 <= count;
            if (elem.disabled && elem.checked) {
                elem.checked = false;
            }
        }
    }

    deleteRow(weapon: string, element: string): void {
        this.doneCounts.get(weapon)!.set(element, 0);
        this.update();
        for (const row of this.table.children) {
            const w = row.querySelector('.found-weapon')?.textContent;
            const e = row.querySelector('.found-element')?.textContent;
            if (w === weapon && e === element) {
                this.table.removeChild(row);
                if (this.table.children.length === 0) {
                    this.tableRoot.classList.add('hidden');
                }
                return;
            }
        }
    }

    reset(): void {
        this.ongoing = { ...ONGOING_INIT };
        this.disableCountUntil(0);
        for (const m of this.doneCounts.values()) {
            for (const k of m.keys()) {
                m.set(k, 0);
            }
        }
        const checked = document.querySelectorAll('input[type="radio"]:checked') as NodeListOf<HTMLInputElement>;
        for (const input of checked) {
            input.checked = false;
        }
        this.table.replaceChildren();
        this.tableRoot.classList.add('hidden');
    }

    prepareCounts(max: number): void {
        while (true) {
            const c = this.countsRoot.lastChild as HTMLElement | null;
            if (!c || c.tagName === 'LEGEND') {
                break;
            }
            this.countsRoot.removeChild(c);
        }
        for (let count = 1; count <= max; count++) {
            const span = document.createElement('span');
            span.className = 'item';
            const input = document.createElement('input');
            input.type = 'radio';
            input.name = 'count';
            input.addEventListener('change', this.onCountClicked.bind(this, count));
            span.appendChild(input);
            const label = document.createElement('label');
            label.textContent = count.toString();
            span.appendChild(label);
            this.countsRoot.appendChild(span);
        }
    }
}

new App();
