function stdin() {
    return null;
}

function prerun() {
    FS.init(stdin, null, null);
}

Module.preRun = Module.preRun || [];
Module.preRun.push(prerun);
console.log('worker: js: pre:', Module);
