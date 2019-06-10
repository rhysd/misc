var MyLibrary = {
    $Lib__postset: 'Lib.init()',
    $Lib: {
        init: function() {
            onmessage = event => {
                console.log('worker: js: message:', event);
                const data = event.data;
                switch (data.type) {
                    case 'init':
                        {
                            console.log('worker: js: init message received:', data);
                            Lib.hello = Module.cwrap('hello', null, ['number']);
                            Lib.buffer = data.buf;
                        }
                        break;
                    case 'input':
                        {
                            const i = data.input;
                            if (typeof i !== 'number') {
                                console.error('input message data is not number:', i);
                                return;
                            }
                            Lib.hello(i);
                        }
                        break;
                    default:
                        console.error('worker: js: invalid data type:', data);
                        break;
                }
            };
            Lib.hi = () => console.log('worker: js: hi!');
            console.log('worker: js: initialization done');
            postMessage({ type: 'init' });
        },
    },
    hi_my_js_lib: function() {
        Lib.hi();
    },
    my_wait_input: function(timeout) {
        console.log('worker: js: my_wait_input:', timeout);
        Atomics.store(Lib.buffer, 0, 0); // Clear
        console.log('worker: js: before wait:', Lib.buffer, Date.now());
        console.log('worker: js: Atomics.wait:', Atomics.wait(Lib.buffer, 0, 0, timeout));
        console.log('worker: js: after wait:', Lib.buffer, Date.now());
    },
};

autoAddDeps(MyLibrary, '$Lib');
mergeInto(LibraryManager.library, MyLibrary);
