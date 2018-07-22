if (!('serviceWorker' in navigator)) {
    throw Exception('This browser does not support ServiceWorker. This site does not work!!');
}

navigator.serviceWorker
    .register('/sw.js')
    .then(function(registration) {
        console.log('ServiceWorker registration successful with scope: ', registration.scope);
    })
    .catch(function(err) {
        console.log('ServiceWorker registration failed: ', err);
    });

function sleep(ms) {
    const req = new XMLHttpRequest();
    req.open('GET', `/sleep-${ms}.txt`, false);
    req.send(null);
}

const input = document.getElementById('sleep-duration');
const btn = document.getElementById('sleep-btn');

btn.addEventListener(
    'click',
    event => {
        const ms = parseInt(input.value, 10);
        if (isNaN(ms)) {
            console.error('Invalid input:', input.value);
            return;
        }
        const prev = Date.now();
        console.log('Sleep start:', prev);
        sleep(ms);
        const now = Date.now();
        console.log('Sleep end:', now, ', Slept', now - prev, 'ms!');
    },
    { passive: true },
);
