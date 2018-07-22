self.addEventListener('install', event => {
    console.log('Install event:', event);
    // Actually do nothing
});

function wait(ms) {
    return new Promise(resolve => {
        console.log('resolve after', ms, 'ms');
        setTimeout(() => {
            console.log('timeout!');
            resolve(new Response('wake up!'));
        }, ms);
    });
}

const MATCH_SLEEP_MS = /sleep-(\d+)\.txt$/;

self.addEventListener('fetch', event => {
    const m = event.request.url.match(MATCH_SLEEP_MS);
    if (m === null) {
        return;
    }
    const ms = parseInt(m[1], 10);
    console.log('sleep fetch event start:', event, 'with timeout', ms, 'ms');
    event.respondWith(wait(ms));
});
