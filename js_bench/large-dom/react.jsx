import React, { useState, useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import parse from 'html-react-parser';

let start = null;

const App = ({html}) => {
    const [[count, result], setTrial] = useState([0, '']);
    const [tree, setTree] = useState(null);

    const handleStart = () => {
        start = Date.now();
        setTree(parse(html));
    };

    useEffect(() => {
        if (start === null) {
            return;
        }
        const end = Date.now();
        const elapsed = end - start;
        console.log(`Trial ${count}: ${elapsed}ms`);
        start = null;
        setTrial([count + 1, `Elapsed: ${elapsed}ms`]);
    });

    return <>
        <button onClick={handleStart}>Run ({count})</button>
        <div>Result: {result}</div>
        {tree}
    </>;
};


fetch('/test.html').then(res => {
    if (!res.ok) {
        throw Error('Could not fetch test.html');
    }
    return res.text();
}).then(text => {
    const root = document.getElementById('root');
    createRoot(root).render(<App html={text}/>);
}).catch(console.error);
