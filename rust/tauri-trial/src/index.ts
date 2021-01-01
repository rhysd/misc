import { invoke } from 'tauri/api/tauri';
import { listen } from 'tauri/api/event';

invoke({ cmd: 'greet', message: 'Hello' });

const countElem = document.getElementById('count');
if (!countElem) {
    throw Error('Element for count was not found');
}
listen('count', event => {
    const count = parseInt(event.payload as string, 10);
    countElem.textContent = count.toString();
});

window.addEventListener(
    'keydown',
    e => {
        // Note: tauri.promisified seems not working? Returned promise seems pending forever even if it is handled in Rust side.
        invoke({ cmd: 'keyPress', ctrl: e.ctrlKey, meta: e.metaKey, key: e.key });
    },
    { passive: true },
);
