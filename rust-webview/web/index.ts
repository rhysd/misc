import {marked} from 'marked';

interface Ipc {
    postMessage(m: string): void;
}

declare global {
    interface Window {
        myMarkdownPreview: MyPreviewApp;
        ipc: Ipc;
    }
}

type MessageFromMain = {
    kind: "content";
    content: string;
};

type MessageToMain = {
    kind: "init";
};

function sendMessage(m: MessageToMain): void {
    window.ipc.postMessage(JSON.stringify(m));
}

marked.setOptions({
    gfm: true,
});

class MyPreviewApp {
    receive(msg: MessageFromMain): void {
        switch (msg.kind) {
        case "content":
            const elem = document.getElementById('preview');
            if (elem === null) {
                console.error("'preview' element is not found");
                return;
            }
            elem.innerHTML = marked.parse(msg.content);
            break;
        default:
            console.error("Unknown message:", msg);
            break;
        }
    }
}

window.myMarkdownPreview = new MyPreviewApp();
sendMessage({kind: "init"});
